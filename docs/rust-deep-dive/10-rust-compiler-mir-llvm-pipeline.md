# Rust 编译器架构与 MIR/LLVM 优化管线

## 研究目标

- 建立 rustc 从源码到机器码的整体模型。
- 理解 HIR、MIR、LLVM IR 分别解决什么问题。
- 知道编译器优化与所有权检查之间的关系。

## rustc 的总体流程

Rust 编译大致可以分为这些阶段：

```text
source
  -> tokens
  -> AST
  -> HIR
  -> type checking
  -> MIR
  -> borrow checking
  -> MIR optimizations
  -> LLVM IR
  -> machine code
  -> linking
```

真实流程更细，包括宏展开、名称解析、trait 求解、单态化、增量编译、代码生成单元等。但这个模型足以帮助理解 Rust 为什么能同时提供高级抽象和底层性能。

## 词法分析、解析与 AST

编译器首先把源代码变成 token，再解析成 AST。AST 保留了较多源代码语法结构。

宏展开也发生在早期阶段。Rust 宏可以生成新的 token，这些 token 继续参与解析、名称解析和类型检查。

如果宏生成的代码有类型错误，错误信息常常会涉及展开后的代码。过程宏质量很大程度取决于是否能提供准确 span 和可读错误。

## HIR

HIR 是 high-level intermediate representation，高层中间表示。它比 AST 更接近编译器后续分析需要的结构，去掉一些纯语法糖，并完成名称解析等工作。

HIR 阶段适合做：

- 类型检查。
- trait 解析和方法查找。
- lint 分析。
- 生成更低层的 MIR。

Rust 的很多用户可见错误，如类型不匹配、方法不存在、trait bound 不满足，会在 HIR 附近的分析中产生。

## 类型检查与 trait 求解

Rust 类型系统需要处理：

- 泛型参数。
- trait bound。
- 关联类型。
- 自动引用和解引用。
- 生命周期约束。
- impl Trait 和 dyn Trait。

示例：

```rust
fn print<T: std::fmt::Display>(value: T) {
    println!("{value}");
}
```

编译器必须证明传入类型满足 `Display`。如果存在多个 impl、关联类型约束或复杂 where 子句，trait 求解会成为编译复杂度的重要来源。

## MIR 是什么

MIR 是 mid-level intermediate representation。它比 HIR 更接近控制流图，简化了 Rust 语法结构，适合做借用检查和优化。

源代码：

```rust
fn add_one(value: i32) -> i32 {
    value + 1
}
```

MIR 会把代码表示成基本块、局部变量、赋值、跳转和返回。它让编译器更容易分析“某个值在哪里被移动”“某个引用在哪里最后使用”“控制流是否经过某个释放点”。

## 借用检查在 MIR 上执行

现代 Rust 借用检查基于 MIR。原因是 MIR 明确表达控制流和使用点，适合非词法生命周期分析。

借用检查需要知道：

- 哪些位置被借用。
- 借用在哪里活跃。
- 哪些路径发生移动。
- 哪些地方可能 Drop。
- 返回引用和输入引用之间有什么区域关系。

这也是为什么 Rust 能接受很多旧式词法生命周期会拒绝的程序。MIR 提供了更精确的控制流基础。

## Drop Elaboration

Rust 的自动析构也会在中间表示中展开。编译器需要在正确路径插入 drop，并确保部分移动、panic 路径、提前返回等场景下资源被正确释放。

```rust
fn example() {
    let a = String::from("a");
    let b = String::from("b");
    println!("{a}{b}");
}
```

编译器要确保 `b` 和 `a` 在离开作用域时按正确顺序释放。如果中间发生 panic，也要遵守栈展开期间的析构规则。

## MIR 优化

在进入 LLVM 前，rustc 会做一些 MIR 级优化，例如：

- 常量传播。
- 简化控制流。
- 消除不必要临时变量。
- 内联某些函数。
- 优化匹配和分支。

MIR 优化知道 Rust 语言语义，因此可以在比 LLVM 更高层的位置做一些更合适的变换。

## 单态化

泛型代码在代码生成前会单态化。编译器为实际使用的类型生成具体实例。

```rust
fn id<T>(value: T) -> T {
    value
}
```

调用 `id::<i32>` 和 `id::<String>` 会产生不同实例。单态化之后，LLVM 看到的是具体类型代码，可以继续做底层优化。

单态化是 Rust 性能的重要来源，也是编译时间和二进制体积的重要来源。

## LLVM 后端

LLVM IR 是更底层的中间表示。LLVM 负责许多目标平台相关和机器级优化：

- 指令选择。
- 寄存器分配。
- 循环优化。
- 向量化。
- 链接时优化。
- 目标平台代码生成。

Rust 借助 LLVM 获得成熟后端和多平台支持。也因此，某些性能问题需要同时理解 Rust 前端生成了什么 IR，以及 LLVM 后端如何优化。

## 增量编译与查询系统

rustc 内部采用查询系统组织编译任务。很多结果可以缓存，增量编译会尽量复用未变化部分。

这对大型项目很重要：

- 改一个函数不应重编译整个世界。
- crate 边界会影响并行和缓存效果。
- 泛型和宏可能扩大重新编译范围。

工程上，合理拆分 crate、控制公共泛型 API、减少大规模宏生成，都能改善编译体验。

## 查看编译器中间结果

一些命令可以帮助研究：

```bash
rustc --emit=mir src/main.rs
rustc --emit=llvm-ir src/main.rs
cargo rustc -- --emit=llvm-ir
```

Nightly 编译器还支持更多 `-Z` 选项，例如打印 MIR、查看优化阶段等。这些工具适合研究，不建议作为普通项目构建流程依赖。

## 常见误解

- Rust 的内存安全检查不是 LLVM 做的，主要在 rustc 前端和 MIR 阶段完成。
- LLVM 优化不理解完整 Rust 所有权模型，它看到的是更低层 IR。
- MIR 不只是优化工具，也是借用检查的重要基础。
- 编译慢往往不是单一原因，泛型、宏、trait 求解、crate 图都会影响。

## 继续研究

- rustc-dev-guide：overview、HIR、MIR、borrow check、monomorphization、codegen。
- Rust Reference：conditional compilation、linkage、ABI。
- Cargo Book：profiles、incremental compilation、workspaces。
- 工具：`cargo expand`、`cargo llvm-lines`、`cargo bloat`、`rustc -Z` nightly 选项。
