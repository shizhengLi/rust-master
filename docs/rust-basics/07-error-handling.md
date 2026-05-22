# 错误处理：Option、Result 与 ? 运算符

## 学习目标

- 理解 Rust 不使用异常作为主要错误处理机制。
- 掌握 `Option<T>` 和 `Result<T, E>`。
- 学会使用 `?` 传播错误。

## Rust 的错误处理思路

Rust 把错误分成两类：

- 可恢复错误：文件不存在、解析失败、网络请求失败等，通常用 `Result<T, E>`。
- 不可恢复错误：数组越界、违反内部不变量等，通常用 `panic!`。

Rust 没有传统异常机制。函数是否可能失败，会体现在返回类型里。

## Option

`Option<T>` 表示一个值可能存在，也可能不存在：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

示例：

```rust
fn first_char(text: &str) -> Option<char> {
    text.chars().next()
}

fn main() {
    match first_char("rust") {
        Some(ch) => println!("{ch}"),
        None => println!("empty string"),
    }
}
```

`Option` 常用于查找结果、可选配置、集合取值等场景。

## Result

`Result<T, E>` 表示成功或失败：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

示例：

```rust
fn parse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    text.parse::<i32>()
}

fn main() {
    match parse_number("42") {
        Ok(number) => println!("{number}"),
        Err(err) => println!("parse failed: {err}"),
    }
}
```

## unwrap 与 expect

`unwrap` 会在 `None` 或 `Err` 时 panic：

```rust
let number = "42".parse::<i32>().unwrap();
```

`expect` 可以提供更清晰的错误信息：

```rust
let number = "42"
    .parse::<i32>()
    .expect("expected a valid integer");
```

学习和原型阶段可以适度使用，但业务代码中应该优先显式处理错误或向上传播错误。

## ? 运算符

`?` 用于简化错误传播：

```rust
use std::fs;
use std::io;

fn read_config() -> Result<String, io::Error> {
    let content = fs::read_to_string("config.txt")?;
    Ok(content)
}
```

如果 `read_to_string` 成功，`?` 取出 `Ok` 里的值。如果失败，函数立即返回 `Err`。

可以进一步写短：

```rust
use std::fs;
use std::io;

fn read_config() -> Result<String, io::Error> {
    fs::read_to_string("config.txt")
}
```

但当函数中有多步操作时，`?` 非常有用。

## main 返回 Result

`main` 也可以返回 `Result`：

```rust
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("config.txt")?;
    println!("{content}");
    Ok(())
}
```

`Box<dyn Error>` 可以表示多种错误类型，适合示例、小工具或命令行程序入口。

## Option 的常用方法

```rust
fn main() {
    let name = Some("Rust");

    println!("{}", name.unwrap_or("unknown"));
    println!("{}", name.map(|value| value.len()).unwrap_or(0));
}
```

常见方法包括：

- `unwrap_or`：不存在时使用默认值。
- `map`：存在时转换内部值。
- `and_then`：链式处理可能失败的步骤。

## 常见误区

- 不要把 `unwrap` 当成正常错误处理。
- 能恢复的错误优先用 `Result`，不是 `panic!`。
- `?` 只能用于返回 `Result`、`Option` 或兼容类型的函数。
- `Option` 表示缺失，`Result` 表示成功或失败及失败原因。

## 练习

1. 写一个函数，把字符串解析成 `u32`，返回 `Result<u32, ParseIntError>`。
2. 写一个函数，返回字符串的第一个字符，空字符串返回 `None`。
3. 用 `?` 读取一个文件并返回内容。
