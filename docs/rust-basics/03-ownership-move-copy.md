# 所有权、移动与复制

## 学习目标

- 理解所有权规则。
- 区分移动语义和复制语义。
- 知道 `String` 与整数等基础类型在赋值时的差异。

## 所有权规则

Rust 的所有权系统有三条核心规则：

1. Rust 中每个值都有一个所有者。
2. 同一时间一个值只能有一个所有者。
3. 当所有者离开作用域，值会被释放。

示例：

```rust
fn main() {
    {
        let name = String::from("Rust");
        println!("{name}");
    } // name 离开作用域，String 占用的堆内存被释放
}
```

这套规则让 Rust 不需要垃圾回收器，也能自动管理内存。

## 栈与堆

简单类型如整数、布尔值通常存放在栈上：

```rust
let x = 5;
let y = x;
println!("{x}, {y}");
```

这段代码可以运行，因为 `i32` 实现了 `Copy`。赋值时复制了一份值。

`String` 这类拥有堆内存的数据不同：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}"); // 编译错误：s1 已经被移动
    println!("{s2}");
}
```

`s1` 被移动到 `s2` 后，`s1` 不再有效。这样可以避免两个变量在离开作用域时重复释放同一块堆内存。

## 移动

移动不是深拷贝。对 `String` 来说，移动通常只是复制指针、长度和容量等元信息，然后让原变量失效。

```rust
fn main() {
    let message = String::from("hello");
    take(message);
    // println!("{message}"); // 编译错误
}

fn take(value: String) {
    println!("{value}");
}
```

把 `message` 传给函数，也会发生移动。函数参数 `value` 成为新的所有者。

## 克隆

如果确实需要复制堆数据，可以使用 `clone`：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1}, {s2}");
}
```

`clone` 可能分配新内存并复制内容，所以成本比移动更高。看到 `clone` 时，应该意识到这里发生了显式复制。

## Copy Trait

实现了 `Copy` 的类型在赋值或传参时会复制，不会移动。常见 `Copy` 类型包括：

- 整数、浮点数、布尔值、字符。
- 只包含 `Copy` 类型的元组。
- 不可变引用 `&T`。

`String`、`Vec<T>` 等拥有堆资源的类型通常不实现 `Copy`。

```rust
fn main() {
    let x = 10;
    print_number(x);
    println!("{x}"); // 仍然可用
}

fn print_number(n: i32) {
    println!("{n}");
}
```

## 返回所有权

函数可以把所有权返回给调用者：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = add_world(s1);

    println!("{s2}");
}

fn add_world(mut value: String) -> String {
    value.push_str(", world");
    value
}
```

但如果每次只是想读取值，来回移动所有权会很麻烦。下一篇会介绍借用和引用，它们是 Rust 日常开发中更常用的方式。

## 常见误区

- 移动不是深拷贝，而是所有权转移。
- 编译器不允许使用已经被移动的变量。
- 不要为了绕过所有权错误到处写 `clone`；先判断是否可以借用。
- `Copy` 是隐式复制，`Clone` 是显式复制。

## 练习

1. 写一个函数接收 `String` 并打印它，观察调用后原变量是否还能使用。
2. 把上面的例子改成使用 `clone`，观察两个变量都可用。
3. 用 `i32` 重复同样实验，理解 `Copy` 的行为。
