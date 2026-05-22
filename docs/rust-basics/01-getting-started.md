# 认识 Rust：语言定位、工具链与第一个程序

## 学习目标

- 理解 Rust 适合解决什么问题。
- 知道 `rustup`、`rustc`、`cargo` 分别负责什么。
- 能创建并运行一个最小 Rust 项目。

## Rust 是什么

Rust 是一门系统级编程语言，目标是在接近 C/C++ 性能的同时，提供更强的内存安全和并发安全。它最大的特点是把许多原本会在运行时才暴露的问题提前到编译期检查，例如悬垂引用、数据竞争、重复释放内存等。

Rust 适合这些场景：

- 命令行工具和后端服务。
- 高性能网络程序。
- 嵌入式和操作系统相关开发。
- WebAssembly。
- 对稳定性和资源控制要求较高的基础设施软件。

Rust 不会自动回收内存，也没有传统意义上的垃圾回收器。它依靠所有权、借用和生命周期规则，让编译器判断一段内存何时可用、何时释放、谁可以读取或修改。

## 工具链

Rust 常用工具链包括：

- `rustup`：安装和管理 Rust 工具链版本。
- `rustc`：Rust 编译器。
- `cargo`：项目管理、依赖管理、构建、测试和发布工具。
- `rustfmt`：代码格式化工具。
- `clippy`：静态检查工具，能发现常见低质量写法。

日常开发中最常使用的是 `cargo`，而不是直接调用 `rustc`。

## 创建第一个项目

```bash
cargo new hello-rust
cd hello-rust
cargo run
```

`cargo new` 会生成一个标准项目：

```text
hello-rust/
├── Cargo.toml
└── src/
    └── main.rs
```

`Cargo.toml` 是项目配置文件，记录包名、版本、依赖和构建配置。`src/main.rs` 是二进制程序入口。

默认代码通常类似：

```rust
fn main() {
    println!("Hello, world!");
}
```

`main` 是程序入口函数。`println!` 后面的感叹号表示它是一个宏，不是普通函数。

## 常用 Cargo 命令

```bash
cargo build      # 编译项目
cargo run        # 编译并运行
cargo check      # 只做快速检查，不生成最终可执行文件
cargo test       # 运行测试
cargo fmt        # 格式化代码
cargo clippy     # 执行更严格的静态检查
```

学习阶段推荐频繁使用 `cargo check`。它比完整构建更快，可以快速发现类型错误、借用错误和语法错误。

## Debug 与 Release

默认 `cargo build` 生成 debug 构建，编译快，运行速度不是最优。

```bash
cargo build --release
```

release 构建会启用优化，适合性能测试和发布。但它编译更慢，调试信息也更少。

## 常见误区

- 不要一开始就把 Rust 当成“语法更现代的 C++”。Rust 的核心是所有权模型。
- 不要直接跳过编译器错误。Rust 编译器错误通常很有价值，能告诉你概念哪里没理解到位。
- 不要只看语法。必须通过小程序练习所有权、借用、错误处理和 trait。

## 练习

1. 创建一个新项目 `guess-number`，运行默认程序。
2. 修改 `main.rs`，打印你的名字和当前学习目标。
3. 分别运行 `cargo check`、`cargo run`、`cargo build --release`，观察生成的 `target` 目录变化。
