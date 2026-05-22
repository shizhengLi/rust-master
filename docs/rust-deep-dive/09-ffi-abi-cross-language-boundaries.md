# FFI、ABI 与跨语言边界设计

## 研究目标

- 理解 FFI 不只是语法互调，还包含 ABI、所有权和错误边界。
- 掌握 Rust 与 C 交互时的基础表示和安全约束。
- 学会设计清晰的跨语言 API 边界。

## FFI 与 ABI

FFI 是 foreign function interface，指不同语言之间互相调用。ABI 是 application binary interface，描述函数调用约定、参数传递、返回值、符号命名、类型布局等底层规则。

Rust 与 C 交互时，常使用外部块声明 C 函数。下面示例使用 Rust 2024 兼容写法：

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}
```

`extern "C"` 表示使用 C ABI，`unsafe extern` 表示声明者必须保证外部函数签名正确。调用外部函数是不安全的，因为 Rust 编译器无法验证外部代码是否满足声明。

```rust
fn main() {
    let value = unsafe { abs(-3) };
    println!("{value}");
}
```

## 导出 Rust 函数给 C

Rust 函数默认符号名会被改编。要导出稳定符号，可以使用：

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`#[unsafe(no_mangle)]` 保留函数名，`extern "C"` 使用 C 调用约定。在 Rust 2024 中，`no_mangle` 属于 unsafe attribute，因为导出未改编符号可能和其他符号冲突。

通常还需要在 `Cargo.toml` 配置库类型：

```toml
[lib]
crate-type = ["cdylib", "staticlib"]
```

`cdylib` 用于生成动态库，`staticlib` 用于静态链接。

## FFI 安全类型

跨 FFI 边界应使用 C 兼容类型：

- `i32`、`u32`、`usize` 等基础整数需注意平台宽度。
- `#[repr(C)]` 结构体。
- 裸指针 `*const T`、`*mut T`。
- C 字符串指针 `*const c_char`。

不要直接在 C ABI 中暴露 `String`、`Vec<T>`、trait object、闭包、普通 Rust enum 等 Rust 专有布局类型。

```rust
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn distance_squared(point: Point) -> i32 {
    point.x * point.x + point.y * point.y
}
```

## 字符串边界

C 字符串通常是以 `\0` 结尾的字节序列。Rust 的 `String` 是 UTF-8、拥有所有权且包含长度和容量。两者不能直接等同。

从 C 接收字符串：

```rust
use std::ffi::CStr;
use std::os::raw::c_char;

pub unsafe fn read_name(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    Some(c_str.to_string_lossy().into_owned())
}
```

这里 unsafe 前提包括：指针非空、指向有效 NUL 结尾字符串、内存在调用期间有效。

把 Rust 字符串传给 C 时可使用 `CString`，它保证内部没有 NUL 字节并添加结尾 NUL。

## 所有权边界

跨语言边界最容易出错的是谁分配、谁释放。

一种常见设计是成对提供创建和释放函数：

```rust
use std::ffi::CString;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn make_message() -> *mut c_char {
    CString::new("hello")
        .unwrap()
        .into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_message(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = unsafe { CString::from_raw(ptr) };
    }
}
```

`into_raw` 把所有权交给调用方，调用方必须把指针传回 `free_message`。不能用 C 的 `free` 释放 Rust 分配的内存。

## 不要跨 FFI unwind

Rust panic 不应跨越不支持 unwind 的 FFI 边界。C++ 异常也不应随意穿过 Rust 栈帧。跨语言边界应把错误转成显式返回值。

常见 C 风格 API：

```rust
#[repr(C)]
pub enum Status {
    Ok = 0,
    InvalidInput = 1,
    InternalError = 2,
}

#[unsafe(no_mangle)]
pub extern "C" fn do_work(input: i32, output: *mut i32) -> Status {
    if output.is_null() {
        return Status::InvalidInput;
    }

    unsafe {
        *output = input * 2;
    }

    Status::Ok
}
```

通过状态码和输出参数表达失败，调用者不需要理解 Rust 的 `Result`。

## opaque pointer 模式

复杂 Rust 类型可以通过不透明指针暴露：

```rust
pub struct Engine {
    count: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn engine_new() -> *mut Engine {
    Box::into_raw(Box::new(Engine { count: 0 }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn engine_increment(engine: *mut Engine) {
    if let Some(engine) = unsafe { engine.as_mut() } {
        engine.count += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn engine_free(engine: *mut Engine) {
    if !engine.is_null() {
        let _ = unsafe { Box::from_raw(engine) };
    }
}
```

C 侧只持有 `Engine*`，不知道内部布局。Rust 保留实现细节和内存管理能力。

## bindgen 与 cbindgen

工具可以减少手写绑定：

- `bindgen`：根据 C 头文件生成 Rust FFI 绑定。
- `cbindgen`：根据 Rust 代码生成 C/C++ 头文件。

工具能减少机械错误，但不能自动解决所有权、线程安全、错误模型和生命周期设计。边界设计仍然需要人工审查。

## 跨语言 API 设计建议

- 使用小而稳定的 C ABI 表面。
- 不暴露 Rust 专有类型。
- 明确每个指针是否可空、是否拥有所有权、是否可变。
- 提供成对释放函数。
- 不让 panic 或异常跨边界。
- 把错误转成状态码或显式错误对象。
- 对 unsafe 函数写清 safety contract。

## 常见误解

- `extern "C"` 只解决调用约定，`unsafe extern` 也不自动保证内存安全。
- `#[repr(C)]` 只影响布局，不验证字段语义安全。
- 指针非空不代表有效，也不代表对齐正确。
- 跨语言边界不要共享隐含所有权协议。

## 继续研究

- Rustonomicon：FFI、repr、ownership across FFI。
- Rust Reference：external blocks、ABI、type layout。
- bindgen 和 cbindgen 文档。
- C ABI、平台 calling convention 和动态链接文档。
