# 结构体、枚举与模式匹配

## 学习目标

- 使用结构体组织相关数据。
- 使用枚举表达有限状态。
- 掌握 `match` 和 `if let` 的基本用法。

## 结构体

结构体用于给一组相关字段命名：

```rust
struct User {
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    println!("{} <{}>", user.name, user.email);
}
```

字段默认不可变。如果需要修改字段，整个结构体绑定必须是 `mut`：

```rust
let mut user = User {
    name: String::from("Alice"),
    email: String::from("alice@example.com"),
    active: true,
};

user.active = false;
```

Rust 不支持只把某个字段标记为可变。

## 字段初始化简写

当变量名和字段名相同时，可以简写：

```rust
struct User {
    name: String,
    email: String,
}

fn build_user(name: String, email: String) -> User {
    User { name, email }
}
```

## 结构体更新语法

可以基于已有结构体创建新值：

```rust
struct Settings {
    theme: String,
    page_size: u32,
    notifications: bool,
}

fn main() {
    let default = Settings {
        theme: String::from("light"),
        page_size: 20,
        notifications: true,
    };

    let custom = Settings {
        theme: String::from("dark"),
        ..default
    };

    println!("{}", custom.theme);
}
```

注意：如果被移动的字段不是 `Copy`，使用 `..default` 后，`default` 可能不再完整可用。

## 方法

使用 `impl` 给结构体定义方法：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect.area());
}
```

`&self` 是 `self: &Self` 的简写，表示方法借用当前实例。

## 枚举

枚举用于表达一个值只能是若干变体之一：

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

枚举变体可以携带数据：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
```

这让枚举非常适合表达状态机、命令、协议消息和错误类型。

## match

`match` 会穷尽处理所有可能分支：

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn describe(direction: Direction) -> &'static str {
    match direction {
        Direction::Up => "go up",
        Direction::Down => "go down",
        Direction::Left => "go left",
        Direction::Right => "go right",
    }
}
```

如果遗漏某个分支，编译器会报错。

## if let

当只关心一个模式时，可以使用 `if let`：

```rust
fn main() {
    let value = Some(3);

    if let Some(number) = value {
        println!("number={number}");
    }
}
```

这比只为了处理一个分支写完整 `match` 更简洁。

## 常见误区

- 结构体字段可变性由变量绑定决定，不由字段单独决定。
- `match` 必须穷尽所有情况，这是优点，不是麻烦。
- 枚举不是只能表示无数据标签；它的变体可以携带不同形状的数据。
- `if let` 适合单分支，复杂逻辑仍建议使用 `match`。

## 练习

1. 定义一个 `Book` 结构体，包含标题、作者和页数。
2. 给 `Book` 实现一个 `summary` 方法。
3. 定义一个 `TaskStatus` 枚举，包含 `Todo`、`Doing`、`Done`，用 `match` 打印状态说明。
