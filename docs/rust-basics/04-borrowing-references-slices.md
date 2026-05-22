# 借用、引用与切片

## 学习目标

- 理解借用解决了什么问题。
- 区分不可变引用和可变引用。
- 掌握字符串切片和数组切片的基本用法。

## 为什么需要借用

如果函数只是读取一个值，不应该夺走它的所有权。引用允许我们“借用”一个值：

```rust
fn main() {
    let name = String::from("Rust");
    print_name(&name);
    println!("{name}");
}

fn print_name(value: &String) {
    println!("{value}");
}
```

`&name` 创建了一个不可变引用。函数参数 `&String` 表示借用一个 `String`，不拥有它。

## 不可变引用

不可变引用允许读取，不允许修改：

```rust
fn main() {
    let text = String::from("hello");
    let len = length(&text);

    println!("{text}: {len}");
}

fn length(value: &String) -> usize {
    value.len()
}
```

同一时间可以有多个不可变引用：

```rust
let value = String::from("hello");
let r1 = &value;
let r2 = &value;
println!("{r1}, {r2}");
```

## 可变引用

如果需要通过引用修改值，变量本身和引用都要标记为可变：

```rust
fn main() {
    let mut text = String::from("hello");
    append_world(&mut text);

    println!("{text}");
}

fn append_world(value: &mut String) {
    value.push_str(", world");
}
```

同一时间只能有一个可变引用：

```rust
let mut value = String::from("hello");
let r1 = &mut value;
// let r2 = &mut value; // 编译错误
println!("{r1}");
```

这个规则防止数据竞争和意外修改。

## 引用规则

Rust 引用规则可以简化为：

1. 可以有多个不可变引用。
2. 或者有一个可变引用。
3. 不可变引用和可变引用不能同时活跃。
4. 引用不能比被引用的值活得更久。

示例：

```rust
fn main() {
    let mut value = String::from("hello");

    let r1 = &value;
    println!("{r1}");

    let r2 = &mut value;
    r2.push_str("!");
    println!("{r2}");
}
```

这段代码可以通过，因为 `r1` 在 `println!` 后不再使用。Rust 的非词法生命周期分析能识别这种情况。

## 字符串切片

字符串切片 `&str` 表示字符串中的一段视图：

```rust
fn main() {
    let text = String::from("hello world");
    let hello = &text[0..5];
    let world = &text[6..11];

    println!("{hello}, {world}");
}
```

更常见的函数参数写法是 `&str`，因为它既能接收 `String` 的切片，也能接收字符串字面量：

```rust
fn print_text(text: &str) {
    println!("{text}");
}

fn main() {
    let owned = String::from("owned string");
    print_text(&owned);
    print_text("string literal");
}
```

## 数组切片

数组或 `Vec<T>` 也可以切片：

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let middle = &numbers[1..4];

    println!("{middle:?}");
}
```

切片不拥有数据，只是引用连续的一段元素。

## 常见误区

- `&String` 通常可以改成更通用的 `&str`。
- 可变引用不是“更强的引用”，而是带有独占访问要求。
- 字符串切片索引按字节计算，切到 UTF-8 字符中间会 panic。
- 引用不释放数据，所有者离开作用域时才释放。

## 练习

1. 写一个函数接收 `&str`，返回字符串长度。
2. 写一个函数接收 `&mut String`，在末尾追加感叹号。
3. 对数组 `[10, 20, 30, 40, 50]` 创建一个包含中间三个元素的切片并打印。
