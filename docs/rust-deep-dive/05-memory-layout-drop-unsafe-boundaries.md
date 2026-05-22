# Rust 内存布局、Drop 顺序与 unsafe 边界

## 研究目标

- 理解 Rust 类型布局的稳定和不稳定部分。
- 掌握 Drop 顺序对资源管理的影响。
- 知道 unsafe 边界应该如何收缩和审计。

## 内存布局为什么重要

大多数 Rust 代码不需要关心具体内存布局。但在这些场景中，布局会变成核心问题：

- FFI 跨语言调用。
- 网络协议和二进制文件解析。
- 嵌入式寄存器映射。
- 性能敏感数据结构。
- unsafe 抽象实现。

Rust 默认布局 `repr(Rust)` 不保证字段顺序、填充方式或整体布局稳定。编译器可以为了优化重新安排布局。只要你不依赖具体布局，这很好；一旦需要和外部 ABI 或二进制格式对接，就必须显式选择表示方式。

## repr(Rust) 与 repr(C)

默认结构体：

```rust
struct Point {
    x: i32,
    y: i32,
}
```

如果需要 C 兼容布局，应使用：

```rust
#[repr(C)]
struct CPoint {
    x: i32,
    y: i32,
}
```

`repr(C)` 让字段顺序、对齐和布局遵循 C ABI 的期望，适合 FFI。它不意味着所有内部类型都自动安全，也不代表可以随意把字节转成结构体。字段本身也必须是 FFI 安全的类型。

## 对齐与填充

结构体字段可能因为对齐产生填充：

```rust
#[repr(C)]
struct Example {
    a: u8,
    b: u32,
}
```

`a` 后面通常会有填充字节，使 `b` 位于满足 `u32` 对齐的位置。这些填充字节不应被当作有效数据读取。直接把结构体内存写到磁盘或网络通常不是稳妥协议设计。

如果确实要解析二进制格式，更推荐显式按字节读写，或使用经过审计的 crate。

## 枚举布局与 niche 优化

Rust 会对某些枚举做布局优化。经典例子：

```rust
use std::ptr::NonNull;

fn main() {
    assert_eq!(
        std::mem::size_of::<Option<NonNull<u8>>>(),
        std::mem::size_of::<NonNull<u8>>()
    );
}
```

`NonNull<T>` 不可能为空指针，因此 `Option<NonNull<T>>` 可以用空指针表示 `None`，不需要额外 tag。这类优化称为 niche optimization。

但不能随意假设所有类型的 `Option<T>` 都有相同布局，除非官方文档明确保证。依赖布局优化时必须查证具体类型和表示保证。

## Drop 顺序

局部变量按声明的逆序析构：

```rust
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _a = Guard("a");
    let _b = Guard("b");
} // 先 drop b，再 drop a
```

结构体字段的 Drop 顺序通常按字段声明顺序执行。这个细节会影响资源依赖。例如一个字段持有缓冲区，另一个字段持有引用或句柄时，字段顺序可能影响析构期间能否安全访问资源。

## 手动控制 Drop

可以用 `std::mem::drop` 提前释放：

```rust
fn main() {
    let file = std::fs::File::open("Cargo.toml");
    drop(file);
}
```

`drop` 会消费值并立即运行析构逻辑。常用于提前释放锁：

```rust
use std::sync::Mutex;

fn main() {
    let lock = Mutex::new(vec![1, 2, 3]);

    {
        let mut guard = lock.lock().unwrap();
        guard.push(4);
    } // guard 在这里释放锁

    println!("{lock:?}");
}
```

用代码块缩短 guard 生命周期通常比手写 `drop` 更清晰。

## ManuallyDrop 与 MaybeUninit

unsafe 代码中有时需要控制初始化和析构。

`ManuallyDrop<T>` 禁止自动析构：

```rust
use std::mem::ManuallyDrop;

fn main() {
    let value = ManuallyDrop::new(String::from("hello"));
}
```

`MaybeUninit<T>` 表示可能未初始化的内存：

```rust
use std::mem::MaybeUninit;

let mut value = MaybeUninit::<u32>::uninit();
value.write(42);
let initialized = unsafe { value.assume_init() };
```

这些工具非常底层。错误使用会造成未定义行为。普通业务代码不应使用它们。

## unsafe 的含义

`unsafe` 不会关闭 Rust 的全部检查。它允许执行几类编译器无法静态证明安全的操作，例如：

- 解引用裸指针。
- 调用 unsafe 函数。
- 访问或修改可变静态变量。
- 实现 unsafe trait。
- 访问 union 字段。

`unsafe` 的正确理解是：“这段代码包含编译器不能完全验证的前提，开发者必须手动维护这些前提。”

## unsafe 边界设计

好的 unsafe 代码应该满足：

- unsafe 块尽量小。
- 对外暴露安全 API。
- 用注释明确 safety invariant。
- 所有不变量都能由安全 API 维护。
- 用测试、Miri、fuzzing 等工具辅助验证。

示例模式：

```rust
pub fn get_two_mut<T>(slice: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a == b || a >= slice.len() || b >= slice.len() {
        return None;
    }

    if a < b {
        let (left, right) = slice.split_at_mut(b);
        Some((&mut left[a], &mut right[0]))
    } else {
        let (left, right) = slice.split_at_mut(a);
        Some((&mut right[0], &mut left[b]))
    }
}
```

这个版本不需要 unsafe，因为标准库已经提供了 `split_at_mut`。优先使用安全 API，只有确实无法表达时才自己写 unsafe。

## 未定义行为

Rust 的未定义行为包括但不限于：

- 解引用无效、未对齐或悬垂指针。
- 制造违反别名规则的引用。
- 读取未初始化内存。
- 违反类型有效值范围。
- 数据竞争。

unsafe 代码的风险在于，编译器会基于“未定义行为不会发生”的假设优化程序。一旦违反前提，错误可能表现为随机崩溃、数据损坏或看似无关的逻辑异常。

## 常见误解

- `repr(C)` 不等于可以安全地随便转字节。
- `unsafe` 不代表代码一定不安全，也不代表调用者可以不遵守规则。
- Drop 顺序是资源设计的一部分，不是无关细节。
- `transmute` 是最后手段，不是通用转换工具。

## 继续研究

- Rust Reference：type layout、destructors、undefined behavior。
- Rustonomicon：repr、drop check、uninitialized memory、aliasing。
- Miri：检测未定义行为的解释器工具。
- Unsafe Code Guidelines：Rust unsafe 代码语义讨论。
