# async/await、Future 与执行器模型

## 研究目标

- 理解 Rust async 不是创建线程，而是生成状态机。
- 掌握 `Future`、`Waker`、executor 的基本关系。
- 知道 async 代码中所有权、生命周期和 Send 约束为何常见。

## async 的核心模型

Rust 的 `async fn` 会返回一个实现了 `Future` 的值。调用 async 函数本身并不会立即执行完整逻辑，它只是构造一个 future。

```rust
async fn fetch() -> String {
    String::from("data")
}

fn main() {
    let future = fetch();
    // future 还没有被执行到完成
}
```

future 必须被执行器轮询，才会向前推进。常见执行器包括 Tokio、async-std、smol，也可以在测试或嵌入式环境中使用专门执行器。

## Future Trait

简化后的 `Future` trait 可以理解为：

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`poll` 返回两种状态：

- `Poll::Ready(value)`：计算完成。
- `Poll::Pending`：暂时不能完成，稍后再来轮询。

执行器反复轮询 future。当 future 因等待 IO、定时器或其他事件无法继续时，返回 `Pending`，并通过 `Waker` 告诉执行器将来什么时候再唤醒它。

## await 做了什么

`.await` 会在当前 future 内等待另一个 future 完成：

```rust
async fn handle() -> usize {
    let text = read_text().await;
    text.len()
}

async fn read_text() -> String {
    String::from("hello")
}
```

编译器会把 `handle` 转换成状态机。`read_text().await` 是一个可能暂停的位置。暂停时，当前函数的局部变量需要被保存到 future 对象内部，等唤醒后继续执行。

## async 状态机

可以把 async 函数想象成枚举状态机：

```rust
enum HandleFuture {
    Start,
    WaitingReadText,
    Done,
}
```

真实生成代码更复杂，但关键直觉是：跨越 `.await` 的局部变量会成为 future 状态的一部分。这也是为什么 async 代码经常遇到所有权和生命周期问题。

## Pin 的作用

某些 future 内部可能自引用：状态机里一个字段引用另一个字段。这样的对象一旦被移动，内部引用就可能失效。`Pin` 用于表达“这个值不能再被随意移动”的约束。

普通用户很少需要手写 `Pin`，但理解它有助于解释为什么 `Future::poll` 的接收者是 `Pin<&mut Self>`。async/await 让这些复杂性大多被编译器和运行时封装起来。

## Waker 与唤醒

当 future 返回 `Pending` 时，它必须确保在将来可以继续时调用 waker：

```text
executor poll future
future waits for IO
future stores waker
future returns Pending
IO ready
waker wakes task
executor polls future again
```

如果 future 返回 `Pending` 但没有正确安排唤醒，任务可能永远卡住。执行器和 IO reactor 的配合负责处理这些细节。

## 执行器模型

执行器负责调度任务。一个常见模型是：

1. 任务队列保存可运行 future。
2. 执行器 poll 某个任务。
3. 如果 Ready，任务完成。
4. 如果 Pending，任务让出执行权。
5. 外部事件通过 waker 把任务放回队列。

这和操作系统线程不同。async 任务通常是协作式调度：只有在 `.await` 等挂起点才会让出执行权。一个没有 await 的长 CPU 循环会阻塞同一执行器线程上的其他任务。

## Tokio 示例

```rust
#[tokio::main]
async fn main() {
    let task = tokio::spawn(async {
        "hello"
    });

    let result = task.await.unwrap();
    println!("{result}");
}
```

`tokio::spawn` 通常要求 future 是 `Send + 'static`，因为任务可能在线程池中被移动到其他线程执行，并且执行器不能依赖当前栈帧里的短生命周期引用。

## Send 约束常见来源

下面的模式容易出问题：

```rust
use std::rc::Rc;

async fn work() {
    let value = Rc::new(1);
    some_async().await;
    println!("{value}");
}

async fn some_async() {}
```

`Rc<T>` 不是 `Send`。如果 `value` 跨越 `.await` 存活，那么整个 future 可能不是 `Send`。在多线程执行器中，这类 future 不能被 `spawn`。

修复方式取决于需求：

- 使用 `Arc<T>` 替代 `Rc<T>`。
- 让非 Send 值不跨越 `.await`。
- 使用单线程执行器或 `spawn_local`。

## async 与借用

跨 await 持有借用也需要谨慎：

```rust
async fn print_later(text: &str) {
    wait().await;
    println!("{text}");
}

async fn wait() {}
```

这个 future 的生命周期依赖 `text`。如果要把它放入要求 `'static` 的任务中，就不能借用当前栈上的字符串。常见做法是传入拥有所有权的 `String` 或 `Arc<str>`。

## 阻塞操作

async 代码中不能随意执行阻塞操作：

```rust
std::thread::sleep(std::time::Duration::from_secs(1));
```

这会阻塞执行器线程。应使用运行时提供的异步版本：

```rust
tokio::time::sleep(std::time::Duration::from_secs(1)).await;
```

文件 IO、数据库驱动、HTTP 客户端也应选择 async 兼容版本，或者放到专门的 blocking 线程池。

## 常见误解

- `async fn` 调用后不会自动跑完，必须被 await 或 spawn。
- async 不等于并行；它主要解决等待期间让出执行权。
- `.await` 是可能暂停点，跨越它的变量会影响 future 类型。
- `Send + 'static` 错误通常来自任务调度模型，不是编译器无理限制。

## 继续研究

- Rust Async Book：Future、task wakeups、executor。
- Rust Reference：async functions、async blocks、await expressions。
- Tokio 文档：runtime、task、spawn、spawn_blocking。
- futures crate：FutureExt、Stream、select、join。
