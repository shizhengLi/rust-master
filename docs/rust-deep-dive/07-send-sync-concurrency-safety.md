# Send、Sync 与并发安全抽象

## 研究目标

- 理解 `Send` 和 `Sync` 如何表达跨线程安全。
- 区分所有权转移、共享引用和内部可变性。
- 掌握 Rust 并发抽象背后的类型约束。

## Rust 并发安全的基础

Rust 的并发安全不是来自某个单独的锁库，而是所有权、借用和类型系统共同作用的结果。核心原则是：

- 数据可以被移动到另一个线程，但必须安全。
- 数据可以被多个线程共享引用，但共享访问必须安全。
- 可变共享需要同步原语或原子操作。

`Send` 和 `Sync` 是两个标记 trait，用于表达这些性质。

## Send

`Send` 表示一个类型的值可以安全地转移到另一个线程。

```rust
use std::thread;

fn main() {
    let text = String::from("hello");

    let handle = thread::spawn(move || {
        println!("{text}");
    });

    handle.join().unwrap();
}
```

`String` 是 `Send`，所以可以通过 `move` 闭包转移到新线程。

反例是 `Rc<T>`：

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let value = Rc::new(1);

    // thread::spawn(move || {
    //     println!("{value}");
    // });
}
```

`Rc<T>` 的引用计数更新不是线程安全的，所以它不是 `Send`。

## Sync

`Sync` 表示一个类型可以安全地被多个线程通过共享引用访问。形式化地说，如果 `&T` 是 `Send`，那么 `T` 是 `Sync`。

`i32`、`String`、`Vec<T>` 在合适条件下都是 `Sync`，因为多个线程共享不可变引用读取它们是安全的。

但 `RefCell<T>` 不是 `Sync`：

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(1);
    *value.borrow_mut() += 1;
}
```

`RefCell<T>` 的借用计数是非线程安全的运行时检查，不能被多个线程同时访问。

## Arc 与 Mutex

跨线程共享所有权通常使用 `Arc<T>`：

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let value = Arc::new(String::from("shared"));
    let cloned = Arc::clone(&value);

    let handle = thread::spawn(move || {
        println!("{cloned}");
    });

    println!("{value}");
    handle.join().unwrap();
}
```

如果需要跨线程修改共享数据，使用 `Mutex<T>`：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}
```

`Arc<Mutex<T>>` 的含义是：多个线程共享同一个所有权句柄，每次修改前先获得互斥锁。

## RwLock 与读多写少

`RwLock<T>` 允许多个读者或一个写者：

```rust
use std::sync::RwLock;

fn main() {
    let value = RwLock::new(vec![1, 2, 3]);

    {
        let read = value.read().unwrap();
        println!("{}", read.len());
    }

    {
        let mut write = value.write().unwrap();
        write.push(4);
    }
}
```

读多写少场景中，`RwLock` 可能比 `Mutex` 更合适。但具体性能取决于锁实现、竞争程度和临界区大小。

## 原子类型

简单计数器可以使用原子类型：

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.load(Ordering::Relaxed));
}
```

原子操作避免锁，但内存序选择很重要。`Relaxed` 只保证原子性，不建立跨线程同步顺序。复杂并发算法应谨慎使用原子并配合测试和模型检查。

## Channel

另一种并发思路是消息传递：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();
    });

    let message = rx.recv().unwrap();
    println!("{message}");
}
```

channel 通过移动消息所有权减少共享状态。很多系统可以优先用消息传递表达工作流，只在必要时共享可变状态。

## 自动实现与 unsafe impl

`Send` 和 `Sync` 通常由编译器自动推导。如果类型的所有字段都是 `Send`，该类型一般也是 `Send`；`Sync` 也类似。

手写 `unsafe impl Send` 或 `unsafe impl Sync` 意味着你向编译器承诺这个类型满足跨线程安全不变量：

```rust
struct MyPointer(*mut u8);

// unsafe impl Send for MyPointer {}
```

这非常危险。除非你完全理解内部别名、生命周期、同步和释放规则，否则不要手写这些实现。

## async 中的 Send

异步任务也常遇到 `Send`：

```rust
tokio::spawn(async move {
    // future 必须 Send + 'static
});
```

如果 future 跨 `.await` 保存了非 Send 值，整个 future 就不是 Send。解决方式包括改用 `Arc`、缩小非 Send 值作用域、使用本地任务执行器。

## 常见误解

- `Arc<T>` 只提供线程安全引用计数，不让 `T` 自动可变。
- `Mutex<T>` 保护的是数据访问，不是让逻辑自动无死锁。
- `Send` 是所有权跨线程转移，`Sync` 是共享引用跨线程访问。
- `RefCell<T>` 是单线程内部可变性，不是线程同步工具。

## 继续研究

- Rustonomicon：Send and Sync。
- Rust Book：fearless concurrency。
- Rust Reference：marker traits、undefined behavior、data races。
- Loom：并发模型测试工具。
