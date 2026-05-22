# 函数、控制流与模块组织

## 学习目标

- 掌握 Rust 函数定义和返回值写法。
- 熟悉 `if`、`loop`、`while`、`for`。
- 理解模块、可见性和文件组织的基本规则。

## 函数

Rust 函数使用 `fn` 定义，参数必须标注类型：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("{result}");
}
```

函数返回值类型写在 `->` 后。函数体最后一个表达式如果没有分号，就作为返回值。

也可以显式使用 `return`：

```rust
fn abs(value: i32) -> i32 {
    if value < 0 {
        return -value;
    }

    value
}
```

Rust 风格通常偏向使用尾表达式，遇到提前返回时再用 `return`。

## 语句与表达式

Rust 区分语句和表达式：

- 语句执行动作，不返回可用值。
- 表达式计算并产生值。

```rust
fn main() {
    let x = {
        let y = 3;
        y + 1
    };

    println!("{x}");
}
```

代码块 `{ ... }` 是表达式，最后的 `y + 1` 没有分号，所以整个代码块值为 `4`。

## if 表达式

`if` 可以作为表达式使用：

```rust
fn main() {
    let score = 85;
    let level = if score >= 60 { "pass" } else { "fail" };

    println!("{level}");
}
```

所有分支必须返回同一类型：

```rust
let value = if true { 1 } else { 0 };
```

## loop

`loop` 表示无限循环，可以用 `break` 返回值：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");
}
```

## while

`while` 适合条件循环：

```rust
fn main() {
    let mut n = 3;

    while n > 0 {
        println!("{n}");
        n -= 1;
    }
}
```

## for

遍历集合时优先使用 `for`：

```rust
fn main() {
    let numbers = [10, 20, 30];

    for number in numbers {
        println!("{number}");
    }
}
```

范围也可以遍历：

```rust
for n in 1..=5 {
    println!("{n}");
}
```

`1..5` 不包含 5，`1..=5` 包含 5。

## 模块基础

模块用于组织代码和控制可见性：

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = math::add(1, 2);
    println!("{result}");
}
```

默认情况下，模块中的项是私有的。需要对外使用时加 `pub`。

## 文件组织

当项目变大时，可以把模块拆到文件中：

```text
src/
├── main.rs
└── math.rs
```

`src/main.rs`：

```rust
mod math;

fn main() {
    println!("{}", math::add(1, 2));
}
```

`src/math.rs`：

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`mod math;` 告诉编译器加载同级的 `math.rs` 文件。

## use

`use` 可以把路径引入当前作用域：

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("rust", 1);
}
```

不要为了省几个字符过度使用通配符导入。清晰的路径更利于阅读。

## 常见误区

- 函数参数必须标注类型，局部变量通常可以让编译器推断。
- 尾表达式不能带分号；带分号就变成语句。
- `if` 分支类型必须一致。
- 模块内的函数默认私有，需要 `pub` 才能被外部访问。

## 练习

1. 写一个函数判断数字是否为偶数。
2. 用 `for` 打印 1 到 10 的平方。
3. 创建一个 `math` 模块，提供 `add` 和 `multiply` 两个公开函数。
