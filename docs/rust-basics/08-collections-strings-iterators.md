# 集合类型、字符串与迭代器入门

## 学习目标

- 掌握 `Vec<T>`、`HashMap<K, V>` 的基本用法。
- 理解 `String` 和 `&str` 的区别。
- 使用迭代器进行常见数据处理。

## Vec

`Vec<T>` 是可增长数组，所有元素类型相同：

```rust
fn main() {
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("{numbers:?}");
}
```

也可以用宏创建：

```rust
let numbers = vec![10, 20, 30];
```

读取元素有两种常见方式：

```rust
let numbers = vec![10, 20, 30];

let first = numbers[0];
let maybe_first = numbers.get(0);

println!("{first}");
println!("{maybe_first:?}");
```

索引越界会 panic，`get` 返回 `Option<&T>`，更适合处理不确定位置。

## 遍历 Vec

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    for number in &numbers {
        println!("{number}");
    }
}
```

如果需要修改元素：

```rust
fn main() {
    let mut numbers = vec![10, 20, 30];

    for number in &mut numbers {
        *number += 1;
    }

    println!("{numbers:?}");
}
```

`*number` 是解引用，用于修改引用指向的值。

## HashMap

`HashMap<K, V>` 存储键值对：

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 85);

    println!("{scores:?}");
}
```

读取值：

```rust
let name = String::from("Alice");
let score = scores.get(&name);
```

`get` 返回 `Option<&V>`，因为键可能不存在。

## entry API

`entry` 常用于“如果不存在就插入”：

```rust
use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();

    for word in ["rust", "go", "rust"] {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{counts:?}");
}
```

这是统计词频、累加分组数据时非常常用的写法。

## String 与 &str

`String` 是可增长、拥有所有权的 UTF-8 字符串。`&str` 是字符串切片，通常是借用视图。

```rust
fn print_text(text: &str) {
    println!("{text}");
}

fn main() {
    let owned = String::from("hello");
    let literal = "world";

    print_text(&owned);
    print_text(literal);
}
```

函数参数优先写 `&str`，可以同时接收 `String` 和字符串字面量。

追加字符串：

```rust
let mut text = String::from("hello");
text.push(' ');
text.push_str("world");
```

## 字符串遍历

Rust 字符串是 UTF-8 编码，不支持按字符位置直接索引：

```rust
let text = String::from("你好");
// let ch = text[0]; // 编译错误
```

可以按字符遍历：

```rust
for ch in text.chars() {
    println!("{ch}");
}
```

也可以按字节遍历：

```rust
for byte in text.bytes() {
    println!("{byte}");
}
```

## 迭代器

迭代器让集合处理更简洁：

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers
        .iter()
        .map(|n| n * 2)
        .collect();

    println!("{doubled:?}");
}
```

过滤：

```rust
let even: Vec<i32> = numbers
    .iter()
    .copied()
    .filter(|n| n % 2 == 0)
    .collect();
```

求和：

```rust
let total: i32 = numbers.iter().sum();
```

## iter、iter_mut、into_iter

- `iter()`：按不可变引用遍历。
- `iter_mut()`：按可变引用遍历。
- `into_iter()`：消费集合，按值遍历。

```rust
let names = vec![String::from("Alice"), String::from("Bob")];

for name in names.into_iter() {
    println!("{name}");
}

// names 此后不可用
```

## 常见误区

- `Vec<T>` 是动态数组，不等于链表。
- `HashMap::get` 返回 `Option`，因为键可能不存在。
- Rust 字符串不能随意按数字下标取字符。
- 迭代器通常是惰性的，很多操作需要 `collect`、`sum` 等消费方法才会执行。

## 练习

1. 用 `Vec<i32>` 存储 5 个数字，计算总和。
2. 用 `HashMap<&str, i32>` 统计数组中单词出现次数。
3. 把一个数字列表过滤成偶数列表，再把每个偶数乘以 10。
