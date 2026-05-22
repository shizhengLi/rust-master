# Rust 所有权模型的编译期推理机制

## 研究目标

- 从编译期推理角度理解所有权，而不是只记规则。
- 理解移动、借用、Drop 与作用域之间的关系。
- 知道所有权模型如何把内存安全问题转化为类型检查问题。

## 为什么所有权是 Rust 的核心

Rust 的目标不是“写起来像 C++ 但语法更现代”，而是在没有垃圾回收器的前提下提供内存安全。传统系统语言常见问题包括悬垂指针、重复释放、释放后使用、迭代器失效和并发数据竞争。Rust 的做法是把资源访问权放进类型系统，让编译器在程序运行前检查资源是否被正确使用。

所有权模型的关键并不是“变量离开作用域就释放”这么简单，而是编译器会追踪值的所有者、移动路径、借用状态和析构需求。一个值如果拥有堆内存、文件句柄、锁、网络连接等资源，它的所有权转移会影响谁负责最终释放资源。

## 所有权是资源协议

可以把所有权理解为一种资源协议：

1. 创建值时，某个绑定成为所有者。
2. 所有者可以把值移动给另一个绑定或函数参数。
3. 值被移动后，原所有者不能再使用。
4. 所有者离开作用域时，如果值仍然有效，就运行析构逻辑。

示例：

```rust
fn main() {
    let a = String::from("rust");
    let b = a;

    // println!("{a}"); // a 已经失效
    println!("{b}");
}
```

这里不是两个 `String` 同时指向同一块堆内存。编译器认为 `a` 的所有权被移动到 `b`，因此 `a` 不再是可用位置。这样离开作用域时只有 `b` 会负责释放堆内存。

## Move Path：编译器如何看待局部变量

Rust 编译器内部并不只把变量看成一个名字。对于结构体字段、元组元素等位置，它会追踪更细粒度的移动路径。

```rust
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 20,
    };

    let name = user.name;
    println!("{name}");
    println!("{}", user.age);

    // println!("{}", user.name); // name 字段已经被移动
}
```

`user.name` 被移动后，`user.age` 仍然可用，因为 `age` 是独立字段且实现了 `Copy`。但整个 `user` 不再是完整可用的值。这种字段级分析让 Rust 可以在保持安全的同时减少不必要限制。

## Copy 与 Move 的边界

`Copy` 表示按位复制后原值仍可使用。实现 `Copy` 的类型必须没有自定义析构逻辑，也不能拥有需要唯一释放的资源。

```rust
fn main() {
    let x = 10;
    let y = x;

    println!("{x}, {y}");
}
```

`String` 不能实现 `Copy`，因为复制它的指针、长度和容量会制造两个所有者，最终导致重复释放风险。Rust 选择默认移动，而不是默认深拷贝，是因为深拷贝成本不可忽略，也不应该被隐藏。

## Drop 与所有权

当一个拥有资源的值离开作用域时，Rust 会调用它的 `Drop` 实现：

```rust
struct Guard {
    name: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.name);
    }
}

fn main() {
    let _guard = Guard {
        name: String::from("scope"),
    };
} // 这里调用 drop
```

所有权模型必须和 `Drop` 严格配合。如果允许移动后继续使用原变量，就可能让析构逻辑对同一资源执行两次。Rust 编译器通过移动检查保证每个需要析构的值在每条控制流路径上最多被释放一次。

## 部分移动与 Drop 类型

如果类型实现了 `Drop`，从它里面移动字段会受到更严格限制：

```rust
struct User {
    name: String,
}

impl Drop for User {
    fn drop(&mut self) {
        println!("dropping user");
    }
}

fn main() {
    let user = User {
        name: String::from("Alice"),
    };

    // let name = user.name; // 通常会被拒绝
}
```

原因是 `drop(&mut self)` 可能需要访问整个对象。如果某些字段已经被移走，析构函数就可能观察到一个不完整对象。Rust 宁愿拒绝这种情况，除非使用 `Option::take`、`mem::replace` 等方式显式留下有效占位。

## 所有权与控制流

移动检查必须考虑控制流：

```rust
fn main() {
    let text = String::from("hello");
    let condition = true;

    if condition {
        consume(text);
    } else {
        println!("{text}");
    }
}

fn consume(value: String) {
    println!("{value}");
}
```

这段代码可以通过，因为在 `if` 的两个分支中，`text` 只会被使用一次。但如果在 `if` 后继续使用 `text`，编译器会拒绝，因为存在一条路径上它已经被移动。

## 借用是临时访问权

所有权代表资源最终处置权，借用代表临时访问权。

```rust
fn len(text: &String) -> usize {
    text.len()
}

fn main() {
    let text = String::from("hello");
    let n = len(&text);
    println!("{text}: {n}");
}
```

借用没有转移所有权，所以调用后 `text` 仍然可用。编译器要证明的是：借用期间，所有者不会被移动或以冲突方式修改；引用不会比被引用数据活得更久。

## 工程意义

所有权模型让很多资源管理模式变得自然：

- 文件、锁、连接等资源可以用 RAII 自动释放。
- API 可以通过参数类型表达是否消费调用者的数据。
- 不需要手写大量释放逻辑。
- 多线程共享时，所有权会迫使开发者显式选择 `Arc`、`Mutex`、channel 等并发抽象。

设计函数签名时，可以用这个判断：

- 只读：优先 `&T` 或更抽象的 `&str`、`&[T]`。
- 需要修改但不接管：使用 `&mut T`。
- 需要保存或消费：接收 `T`。
- 需要共享所有权：使用 `Rc<T>` 或 `Arc<T>`。

## 常见误解

- 所有权不是运行时引用计数；大多数检查发生在编译期。
- 移动不是深拷贝；通常只是让原位置失效。
- 生命周期标注不会延长对象生命；它只是描述引用关系。
- 到处 `clone` 能绕过问题，但可能隐藏设计不清和性能成本。

## 继续研究

- Rust Reference：destructors、behavior considered undefined、type layout。
- rustc-dev-guide：MIR、borrow checking、drop elaboration。
- Rustonomicon：ownership、aliasing、drop check、unsafe code guidelines。
