# 变量、不可变性与基础类型

## 学习目标

- 理解 Rust 默认不可变的设计。
- 掌握常见标量类型和复合类型。
- 区分变量遮蔽和可变变量。

## 默认不可变

Rust 中变量默认不可变：

```rust
fn main() {
    let x = 5;
    println!("{x}");
    // x = 6; // 编译错误
}
```

如果需要修改变量，必须显式写 `mut`：

```rust
fn main() {
    let mut x = 5;
    x = 6;
    println!("{x}");
}
```

默认不可变让代码更容易推理。看到一个没有 `mut` 的变量，就知道它绑定之后不会被重新赋值。

## 变量遮蔽

Rust 允许用同名变量遮蔽前一个变量：

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");
}
```

这不是修改原变量，而是创建了一个新的绑定。遮蔽可以改变类型，`mut` 不能改变变量类型。

```rust
fn main() {
    let mut value = "42";
    // value = 42; // 编译错误：类型不一致
}
```

## 标量类型

Rust 常见标量类型包括：

- 整数：`i32`、`u32`、`i64`、`usize` 等。
- 浮点数：`f32`、`f64`。
- 布尔值：`bool`。
- 字符：`char`。

示例：

```rust
fn main() {
    let count: u32 = 10;
    let price: f64 = 19.9;
    let enabled: bool = true;
    let letter: char = 'R';

    println!("{count}, {price}, {enabled}, {letter}");
}
```

`char` 表示 Unicode 标量值，不只是 ASCII 字符，所以一个 `char` 可能占用多个字节。

## 整数类型选择

初学时可以遵循简单规则：

- 普通数字优先使用 `i32`。
- 数组索引、集合长度使用 `usize`。
- 明确需要非负并且有接口约束时使用 `u32`、`u64` 等。

不要为了“看起来更节省空间”过早选择很小的整数类型，例如 `u8` 或 `i8`。除非你正在处理二进制协议、图像数据、嵌入式内存等场景。

## 元组

元组可以组合不同类型的值：

```rust
fn main() {
    let user = ("Alice", 18, true);
    let name = user.0;
    let age = user.1;

    println!("{name} is {age}");
}
```

也可以解构：

```rust
fn main() {
    let point = (3, 5);
    let (x, y) = point;

    println!("x={x}, y={y}");
}
```

## 数组

数组长度固定，且所有元素类型相同：

```rust
fn main() {
    let scores = [90, 85, 100];
    println!("{}", scores[0]);
}
```

数组类型可以写成 `[类型; 长度]`：

```rust
let scores: [i32; 3] = [90, 85, 100];
let zeros = [0; 5]; // [0, 0, 0, 0, 0]
```

如果需要动态增长的列表，通常使用 `Vec<T>`，后续集合文章会介绍。

## 常量

常量使用 `const`，必须标注类型：

```rust
const MAX_POINTS: u32 = 100_000;
```

常量和不可变变量不同：常量在整个程序生命周期内有效，不能使用运行时计算结果初始化。

## 常见误区

- `let` 默认不可变，不等于值永远不能变化；可以用遮蔽创建新绑定。
- `mut` 只允许重新赋值，不允许改变变量类型。
- 数组长度固定；动态列表应使用 `Vec<T>`。
- 字符串字面量不是 `String`，而是 `&str`。

## 练习

1. 写一个程序，定义姓名、年龄、是否正在学习 Rust，并打印出来。
2. 用变量遮蔽把字符串 `"2026"` 转成数字 `2026`。
3. 创建一个长度为 5 的整数数组，打印第一个和最后一个元素。
