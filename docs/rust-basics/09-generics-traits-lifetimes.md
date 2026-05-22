# 泛型、Trait 与生命周期基础

## 学习目标

- 理解泛型如何减少重复代码。
- 使用 trait 表达共享行为。
- 建立生命周期的基本直觉。

## 泛型

泛型允许函数、结构体或枚举处理多种类型：

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    let numbers = vec![1, 2, 3];
    let names = vec!["Alice", "Bob"];

    println!("{:?}", first(&numbers));
    println!("{:?}", first(&names));
}
```

`T` 是类型参数。它表示“某个具体类型”，编译器会在使用时推断出来。

## 泛型结构体

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };

    println!("{}, {}", integer.x, float.x);
}
```

如果两个字段允许不同类型，可以使用两个类型参数：

```rust
struct Pair<T, U> {
    left: T,
    right: U,
}
```

## Trait

Trait 定义共享行为，类似“能力约束”：

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
```

使用 trait 约束函数参数：

```rust
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

这表示参数可以是任何实现了 `Summary` 的类型。

## Trait Bound

更完整的泛型写法：

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

多个约束可以用 `+`：

```rust
use std::fmt::Debug;

fn inspect<T: Summary + Debug>(item: &T) {
    println!("{item:?}");
    println!("{}", item.summarize());
}
```

复杂约束可以用 `where`：

```rust
fn compare_and_print<T, U>(left: &T, right: &U)
where
    T: Summary + Clone,
    U: Summary + std::fmt::Debug,
{
    println!("{}", left.summarize());
    println!("{right:?}");
}
```

## 默认实现

Trait 可以提供默认方法：

```rust
trait Summary {
    fn author(&self) -> &str;

    fn summarize(&self) -> String {
        format!("article by {}", self.author())
    }
}
```

实现者只需要提供没有默认实现的方法。

## 生命周期要解决什么

生命周期描述引用之间的有效范围关系。它不会延长任何值的生命，只是让编译器确认引用不会悬垂。

很多时候生命周期可以省略：

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
```

编译器能推断返回值来自参数 `text`。

当有多个输入引用，返回值来源不明显时，需要标注：

```rust
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
```

`'a` 表示返回引用的有效期不能超过 `left` 和 `right` 中较短的那个。

## 结构体中的引用

如果结构体存储引用，需要生命周期参数：

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let text = String::from("Rust is fast and safe.");
    let first = text.split('.').next().unwrap_or("");
    let excerpt = Excerpt { part: first };

    println!("{}", excerpt.part);
}
```

这表示 `Excerpt` 不能比它引用的字符串片段活得更久。

## 常见误区

- 泛型不是运行时动态类型；Rust 通常会在编译期为具体类型生成代码。
- Trait 是行为抽象，不只是接口语法。
- 生命周期标注不会改变对象实际存活时间。
- 遇到生命周期错误时，先检查谁拥有数据、谁借用了数据、返回引用来自哪里。

## 练习

1. 写一个泛型函数，返回切片的最后一个元素。
2. 定义一个 `DisplayName` trait，并为两个结构体实现它。
3. 写一个函数，在两个 `&str` 中返回更短的那个，并添加必要生命周期标注。
