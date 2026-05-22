# 泛型单态化、代码膨胀与性能取舍

## 研究目标

- 理解 Rust 泛型如何在编译期变成具体代码。
- 分析单态化带来的性能收益和代码体积成本。
- 掌握控制泛型暴露和编译时间的工程方法。

## 什么是单态化

Rust 泛型默认使用单态化。编译器会根据实际使用的具体类型，为泛型函数或类型生成专门版本。

```rust
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let a = identity(1_i32);
    let b = identity("rust");
}
```

编译后可以近似理解为生成了两个版本：

```rust
fn identity_i32(value: i32) -> i32 {
    value
}

fn identity_str(value: &str) -> &str {
    value
}
```

真实编译过程更复杂，但直觉是：泛型抽象大多在编译期消除，运行时不需要额外类型信息。

## 性能收益

单态化的直接收益是优化空间大：

- 编译器知道具体类型大小和布局。
- 方法调用可静态解析。
- 小函数更容易内联。
- 分支和边界检查可能被消除。
- 迭代器链常能优化成接近手写循环的代码。

例如：

```rust
fn sum<I>(items: I) -> i32
where
    I: IntoIterator<Item = i32>,
{
    items.into_iter().sum()
}
```

对 `Vec<i32>`、数组、范围等不同输入，编译器可以分别生成针对具体迭代器的代码。

## 代码膨胀

单态化的代价是代码体积和编译时间。一个泛型函数如果被许多类型实例化，就可能生成许多份机器码。

```rust
fn process<T: serde::Serialize>(value: T) {
    // 假设函数内部逻辑很复杂
}
```

如果它在很多模块中被大量类型调用，二进制体积可能增加。尤其当泛型函数体很大、内联层级深、trait bound 复杂时，编译时间也会明显上升。

## 泛型边界设计

不要把泛型扩散到不需要的地方。常见做法是“外层泛型，内层具体”：

```rust
pub fn read_config<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    read_config_impl(path.as_ref())
}

fn read_config_impl(path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

公开 API 接受泛型参数，方便调用者传 `&str`、`String`、`PathBuf` 等；内部实现转成 `&Path`，避免复杂逻辑为每种 `P` 生成一份。

## 泛型参数位置的成本

下面两个函数语义接近，但实例化成本可能不同：

```rust
fn log_generic<T: std::fmt::Display>(value: T) {
    println!("{value}");
}

fn log_dyn(value: &dyn std::fmt::Display) {
    println!("{value}");
}
```

`log_generic` 对不同 `T` 可能生成多份代码，`log_dyn` 通过动态分发共享一份函数体。前者可能更快，后者可能减少代码体积。日志、错误报告、低频路径常常不需要极致静态分发。

## impl Trait 与泛型传播

参数位置的 `impl Trait` 仍然是泛型：

```rust
fn handle(input: impl AsRef<str>) {
    println!("{}", input.as_ref());
}
```

这不是动态分发。它只是隐藏了类型参数名字。对多个具体类型调用仍然会单态化。

返回位置的 `impl Trait` 表示一个隐藏的具体返回类型：

```rust
fn ids() -> impl Iterator<Item = u64> {
    0..100
}
```

这可以避免暴露复杂迭代器类型，同时保留静态分发和优化能力。

## Iterator 链为什么通常很快

Rust 迭代器是泛型抽象。像下面的代码：

```rust
fn total_even_squares(values: &[i32]) -> i32 {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .map(|value| value * value)
        .sum()
}
```

表面上创建了多个适配器，但这些适配器类型在编译期完全可见。优化后常能消除中间结构，生成紧凑循环。这种“零成本抽象”依赖单态化、内联和 LLVM 优化。

但它不是保证。复杂闭包、无法内联边界、动态分发、调试构建都可能影响结果。性能敏感代码应使用基准测试和生成代码分析验证。

## 动态分发作为体积控制工具

当泛型函数体较大而具体类型很多时，可以把热路径和冷路径拆开：

```rust
pub fn parse<T: AsRef<[u8]>>(input: T) -> Result<usize, String> {
    parse_impl(input.as_ref())
}

fn parse_impl(input: &[u8]) -> Result<usize, String> {
    // 大量解析逻辑只生成一份
    Ok(input.len())
}
```

或者使用 trait object：

```rust
fn run(task: &dyn Task) {
    task.execute();
}
```

这种设计牺牲一些静态优化，但可能换来更小二进制和更快编译。

## LTO、代码生成单元与优化配置

发布构建中可以通过 Cargo 配置影响体积和性能：

```toml
[profile.release]
lto = true
codegen-units = 1
strip = true
```

含义：

- `lto`：链接时优化，跨 crate 优化更充分。
- `codegen-units = 1`：减少并行代码生成单元，优化更好但编译更慢。
- `strip`：移除符号信息，减小体积。

这些选项需要按项目目标调整。CLI 工具、嵌入式程序、服务端二进制的优先级可能不同。

## 编译时间管理

泛型和宏会影响编译时间。常见优化方式：

- 避免在公共 API 中暴露过度复杂的泛型类型。
- 大函数内部尽早转成具体类型或 trait object。
- 使用 workspace 拆分稳定模块。
- 减少不必要的 feature 开启。
- 对热路径保留泛型，对冷路径使用动态分发或具体类型。

## 常见误解

- 泛型不是运行时模板解释；大多在编译期实例化。
- `impl Trait` 不自动减少代码膨胀。
- 动态分发不一定慢到不可接受，关键看调用频率和优化边界。
- 零成本抽象不是无需验证；它是设计目标，不是每段代码的无条件结论。

## 继续研究

- rustc-dev-guide：monomorphization、codegen、MIR optimizations。
- Rust Reference：generics、trait bounds、impl Trait。
- Cargo Book：profiles、LTO、codegen-units。
- 工具：`cargo bloat`、`cargo llvm-lines`、`cargo asm`、criterion。
