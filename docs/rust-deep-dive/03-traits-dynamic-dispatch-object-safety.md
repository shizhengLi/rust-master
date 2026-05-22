# Trait 系统、动态分发与对象安全

## 研究目标

- 理解 trait 既是接口抽象，也是类型系统约束。
- 区分静态分发和动态分发。
- 掌握 trait object 为什么需要对象安全规则。

## Trait 的两种角色

Trait 在 Rust 中承担两类角色：

1. 定义共享行为，例如 `Display`、`Iterator`、`Read`。
2. 作为泛型约束，告诉编译器某个类型必须具备哪些能力。

```rust
trait Summary {
    fn summarize(&self) -> String;
}

fn print<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

这里 `T: Summary` 是静态约束。编译器在编译期知道具体类型，可以直接生成针对该类型的代码。

## 静态分发

泛型函数默认使用静态分发：

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

如果分别用 `Article` 和 `Book` 调用，编译器通常会为不同具体类型生成专门版本。好处是调用可以内联和优化；代价是代码体积可能增加。

也可以写成 `impl Trait`：

```rust
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

在参数位置，`impl Summary` 基本等价于匿名泛型参数，仍然是静态分发。

## 动态分发

动态分发通过 trait object 实现：

```rust
trait Draw {
    fn draw(&self);
}

struct Button;

impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}

fn render(component: &dyn Draw) {
    component.draw();
}
```

`dyn Draw` 表示“某个实现了 `Draw` 的具体类型，但当前只通过 `Draw` 接口访问”。编译器不知道具体类型，所以调用方法时需要通过虚表间接分发。

## Trait Object 的内存形态

`&dyn Trait` 是胖指针，通常包含两部分：

- 数据指针：指向具体值。
- 虚表指针：指向该具体类型对这个 trait 的方法表。

这就是动态分发的基础。调用 `component.draw()` 时，程序通过虚表找到正确的方法实现。

常见 trait object 形式：

```rust
&dyn Draw
&mut dyn Draw
Box<dyn Draw>
Arc<dyn Draw + Send + Sync>
```

`Box<dyn Draw>` 拥有堆上的具体对象，适合把不同类型放进同一个集合：

```rust
fn main() {
    let components: Vec<Box<dyn Draw>> = vec![Box::new(Button)];

    for component in components {
        component.draw();
    }
}
```

## 为什么需要对象安全

不是所有 trait 都能变成 `dyn Trait`。能作为 trait object 使用的 trait 需要满足对象安全规则，官方文档现在更常称为 dyn compatibility。

考虑这个 trait：

```rust
trait CloneLike {
    fn clone_like(&self) -> Self;
}
```

如果只有 `&dyn CloneLike`，调用 `clone_like` 应该返回什么具体类型？调用者不知道背后的具体类型，返回 `Self` 无法在 trait object 层面表示。因此这种方法不能用于对象安全的 trait object。

再看泛型方法：

```rust
trait Encode {
    fn encode<T>(&self, value: T);
}
```

泛型方法需要为不同 `T` 生成不同代码，而 trait object 的虚表需要固定方法入口。无限可能的 `T` 无法都放进一个固定虚表。

## 常见对象安全限制

一个适合 `dyn Trait` 的 trait 通常应满足：

- 方法接收者是 `self`、`&self`、`&mut self`、`Box<Self>` 等可分发形式。
- 不能在可通过 trait object 调用的方法中返回裸 `Self`。
- 不能有普通泛型方法。
- 不要求 `Self: Sized` 作为整个 trait 的前提。

可以用 `where Self: Sized` 把不适合动态分发的方法排除出 trait object：

```rust
trait Parser {
    fn parse(&self, input: &str) -> bool;

    fn new() -> Self
    where
        Self: Sized;
}
```

`parse` 可以通过 `dyn Parser` 调用，`new` 只能在具体类型上调用。

## 关联类型与对象安全

带关联类型的 trait 可以作为 trait object，但必须指定关联类型：

```rust
trait Source {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn consume(source: &mut dyn Source<Item = String>) {
    while let Some(item) = source.next() {
        println!("{item}");
    }
}
```

`Iterator` 就是典型例子。`dyn Iterator` 不完整，必须写出 `Item`，例如 `Box<dyn Iterator<Item = i32>>`。

## impl Trait 作为返回值

返回位置的 `impl Trait` 表示返回某个具体类型，只是调用者不知道名字：

```rust
fn numbers() -> impl Iterator<Item = i32> {
    0..10
}
```

注意：一个函数的所有返回路径必须返回同一个具体类型：

```rust
fn choose(flag: bool) -> impl Iterator<Item = i32> {
    if flag {
        0..10
    } else {
        // vec![1, 2, 3].into_iter() // 类型不同，不能直接返回
        10..20
    }
}
```

如果需要根据条件返回不同具体类型，通常使用 `Box<dyn Iterator<Item = i32>>` 或定义枚举包装。

## 静态分发与动态分发的取舍

优先使用静态分发：

- 性能敏感路径。
- 类型集合在编译期明确。
- 希望编译器内联和优化。
- API 不需要异构集合。

考虑动态分发：

- 需要把不同实现放进同一个集合。
- 插件、回调、运行时选择实现。
- 减少泛型暴露和编译时间。
- 二进制体积比极致性能更重要。

动态分发的成本通常是一层间接调用和可能失去内联机会。这个成本不一定大，但应当是有意识的设计选择。

## Trait Coherence 与孤儿规则

Rust 限制 trait 实现以避免冲突。孤儿规则大致要求：为某个类型实现某个 trait 时，trait 或类型至少有一个定义在当前 crate。

```rust
// 不能在你的 crate 中为 Vec<T> 实现 Display，
// 因为 Display 和 Vec 都来自外部 crate。
```

这保证不同 crate 不会为同一 trait/type 组合提供相互冲突的实现。

## 常见误解

- `impl Trait` 不等于动态分发；参数和返回位置语义不同。
- `dyn Trait` 不是“不知道类型”的魔法，它依赖胖指针和虚表。
- 对象安全限制不是任意规则，而是虚表模型的结果。
- trait bound 越多越好并不成立；约束应该表达真实需求。

## 继续研究

- Rust Reference：traits、trait objects、dyn compatibility。
- Rust Book：advanced traits、trait objects。
- rustc-dev-guide：trait solving、method lookup。
- Rustonomicon：send/sync、subtyping and variance。
