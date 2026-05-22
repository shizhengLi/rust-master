# 宏系统：声明宏、过程宏与代码生成

## 研究目标

- 理解 Rust 宏在编译期生成代码的角色。
- 区分声明宏和过程宏。
- 掌握宏适合解决的问题和不适合滥用的边界。

## 为什么需要宏

Rust 是静态类型语言，很多重复模式在运行时抽象之前就已经显得冗长。宏可以在编译期生成代码，用于：

- 减少重复样板。
- 创建小型 DSL。
- 根据语法结构生成实现。
- 实现属性驱动的框架集成。

标准库中的 `println!`、`vec!`、`format!` 都是宏。

```rust
fn main() {
    let values = vec![1, 2, 3];
    println!("{values:?}");
}
```

宏调用带 `!`，说明它不是普通函数调用。

## 声明宏 macro_rules!

声明宏通过模式匹配输入 token 并展开输出 token：

```rust
macro_rules! say_hello {
    () => {
        println!("hello");
    };
}

fn main() {
    say_hello!();
}
```

带参数的例子：

```rust
macro_rules! make_vec {
    ($($item:expr),* $(,)?) => {{
        let mut values = Vec::new();
        $(
            values.push($item);
        )*
        values
    }};
}

fn main() {
    let values = make_vec![1, 2, 3];
    println!("{values:?}");
}
```

`$item:expr` 表示匹配表达式，`$()*` 表示重复匹配。

## 片段说明符

`macro_rules!` 常见片段类型包括：

- `expr`：表达式。
- `ident`：标识符。
- `ty`：类型。
- `path`：路径。
- `pat`：模式。
- `stmt`：语句。
- `block`：代码块。
- `tt`：token tree。

选择合适片段能让宏输入更结构化，也能得到更好的错误信息。

## 宏卫生性

Rust 宏具有一定卫生性，宏内部定义的局部变量通常不会意外捕获调用处变量：

```rust
macro_rules! demo {
    () => {{
        let value = 1;
        value
    }};
}

fn main() {
    let value = 10;
    println!("{}", demo!());
    println!("{value}");
}
```

但宏仍然可能引入难读代码、复杂错误和路径解析问题。编写导出宏时，常用 `$crate` 引用当前 crate：

```rust
#[macro_export]
macro_rules! call_helper {
    () => {
        $crate::helper()
    };
}
```

## 过程宏

过程宏接收 token stream，输出 token stream。它们本质上是编译期运行的 Rust 函数。

过程宏分三类：

- 派生宏：`#[derive(MyTrait)]`。
- 属性宏：`#[my_attribute]`。
- 函数式宏：`my_macro!(...)`。

过程宏通常放在独立 crate 中，并配置：

```toml
[lib]
proc-macro = true
```

## 派生宏

派生宏用于根据结构体或枚举定义生成 trait 实现：

```rust
#[derive(Debug, Clone)]
struct User {
    name: String,
}
```

常见第三方例子包括 `serde_derive`、`thiserror`、`clap` 等。它们读取类型结构，生成序列化、错误实现或 CLI 解析代码。

一个概念化的派生宏流程：

```text
input tokens: struct User { name: String }
parse into syntax tree
inspect fields
generate impl MyTrait for User
return output tokens
```

实际开发通常使用 `syn` 解析输入，用 `quote` 生成输出。

## 属性宏

属性宏可以改写或包裹一个 item：

```rust
#[route(GET, "/users")]
fn users() {}
```

Web 框架常用属性宏把函数注册为路由。测试框架、异步运行时、序列化框架也大量使用属性宏。

属性宏能力很强，但也可能隐藏控制流。使用时应确保团队能理解宏展开后的行为。

## 函数式过程宏

函数式过程宏看起来像声明宏调用，但内部由 Rust 代码处理 token：

```rust
sql!(SELECT * FROM users WHERE id = 1);
```

它适合实现复杂 DSL、编译期校验、生成类型安全接口等。与 `macro_rules!` 相比，过程宏更灵活，但编写和维护成本更高。

## 宏展开与调试

宏问题常见调试方法：

- 使用 `cargo expand` 查看展开代码。
- 给宏生成代码保留清晰路径。
- 避免一次生成过大、过深的代码。
- 在过程宏中提供精确 span 和错误信息。

示例：

```bash
cargo install cargo-expand
cargo expand
```

展开后的代码不一定适合阅读全部细节，但能帮助定位实际生成了什么。

## 何时使用宏

适合使用宏：

- 普通函数和泛型无法表达的语法抽象。
- 需要生成大量结构相似代码。
- 需要读取类型定义并生成实现。
- 框架入口确实需要属性式声明。

不适合使用宏：

- 只是为了少写几行普通函数。
- 业务逻辑复杂但可以用类型和函数表达。
- 团队难以维护的 DSL。
- 会显著恶化编译错误的场景。

## 常见误解

- 宏不是运行时反射；它发生在编译期。
- `macro_rules!` 不是字符串替换，而是 token 模式匹配。
- 过程宏能力强，但不是免费抽象，会增加编译复杂度。
- 宏生成代码也必须满足所有权、借用和类型规则。

## 继续研究

- Rust Reference：macros by example、procedural macros。
- The Little Book of Rust Macros。
- `syn`、`quote`、`proc-macro2` 文档。
- `cargo expand` 用于观察宏展开。
