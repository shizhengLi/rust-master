# Cargo、测试与实用开发工作流

## 学习目标

- 熟悉 Cargo 项目结构。
- 编写单元测试和集成测试。
- 建立基础 Rust 开发工作流。

## Cargo.toml

`Cargo.toml` 是 Rust 项目的配置文件：

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

常见字段：

- `name`：包名。
- `version`：版本号。
- `edition`：Rust 版本风格。
- `dependencies`：运行时依赖。
- `dev-dependencies`：只在测试或示例中使用的依赖。

学习阶段不必急着修改复杂配置，先掌握构建、运行和测试即可。

## 项目结构

二进制项目：

```text
src/
└── main.rs
```

库项目：

```text
src/
└── lib.rs
```

同时提供库和可执行程序：

```text
src/
├── lib.rs
└── main.rs
```

常见约定：

- `src/main.rs`：二进制入口。
- `src/lib.rs`：库入口。
- `tests/`：集成测试。
- `examples/`：示例程序。
- `benches/`：基准测试。

## 添加依赖

可以直接编辑 `Cargo.toml`：

```toml
[dependencies]
serde = "1"
```

也可以使用 Cargo 命令：

```bash
cargo add serde
```

依赖版本会记录在 `Cargo.lock` 中。应用程序通常提交 `Cargo.lock`，库项目是否提交取决于团队约定。

## 单元测试

单元测试通常写在同一个文件里：

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

运行测试：

```bash
cargo test
```

`#[cfg(test)]` 表示测试模块只在测试构建中编译。

## 集成测试

集成测试放在 `tests/` 目录：

```text
tests/
└── math_test.rs
```

示例：

```rust
use demo::add;

#[test]
fn adds_two_numbers() {
    assert_eq!(add(2, 3), 5);
}
```

集成测试像外部用户一样使用你的库，因此适合验证公开 API。

## 常用断言

```rust
assert!(value > 0);
assert_eq!(actual, expected);
assert_ne!(left, right);
```

可以添加说明：

```rust
assert_eq!(actual, expected, "calculation should match expected result");
```

测试 panic：

```rust
#[test]
#[should_panic]
fn panics_on_invalid_input() {
    panic!("invalid input");
}
```

## 文档测试

Rust 支持在文档注释中写示例，并由 `cargo test` 执行：

```rust
/// Adds two numbers.
///
/// ```
/// let result = demo::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

文档测试能保证示例长期可运行。

## 推荐开发循环

日常开发可以采用这个循环：

```bash
cargo fmt
cargo check
cargo test
cargo clippy
```

实际顺序可以根据项目大小调整。大项目里 `cargo check` 更适合频繁运行，`cargo test` 可以在完成一个小功能后运行。

## Clippy

Clippy 能发现很多不必要、低效或不符合习惯的写法：

```bash
cargo clippy
```

如果要让警告变成错误：

```bash
cargo clippy -- -D warnings
```

这常用于 CI，但本地学习时不必一开始就过度严格。

## 常见误区

- `cargo check` 不会生成最终可执行文件，但能快速发现大部分编译错误。
- 单元测试更贴近内部实现，集成测试更贴近用户视角。
- 文档示例可以被测试，不只是注释。
- 不要忽视 `cargo fmt`；统一格式能减少无意义的代码差异。

## 练习

1. 创建一个库项目，写一个 `add` 函数和对应单元测试。
2. 在 `tests/` 目录添加一个集成测试。
3. 运行 `cargo fmt`、`cargo test`、`cargo clippy`，记录每个命令的作用。
