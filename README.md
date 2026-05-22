# Rust 知识文章项目

这个仓库用于系统整理 Rust 语言学习材料，并提供一个可以编译运行的 Rust 学习项目。

## Rust 适合写什么项目

Rust 特别适合这些类型的项目：

1. 命令行工具：启动快、部署简单、容易打包成单个二进制文件。
2. 后端服务：性能好，内存安全，适合长期运行的网络程序。
3. 系统工具：适合处理文件、进程、网络、压缩、编译器工具链等底层任务。
4. WebAssembly：可以把性能敏感逻辑编译到浏览器或边缘环境。
5. 嵌入式和基础设施：适合对资源控制、可靠性和并发安全要求高的场景。

学习 Rust 时，不建议一开始就写大型异步服务或复杂图形项目。更好的路线是先写一个小而完整的 CLI 工具，因为它能覆盖所有权、结构体、枚举、错误处理、文件读写、测试和 Cargo 工作流。

## 学习项目：rusty-tasks

`rusty-tasks` 是一个本地任务清单 CLI。它不依赖外部 crate，数据保存在当前目录的 `.rusty-tasks.txt` 文件中，适合作为第一个完整 Rust 项目。

它练习这些 Rust 知识点：

- `struct` 和 `enum` 建模任务与过滤条件。
- `Result`、`Option` 和 `?` 风格错误处理。
- `Vec<T>`、迭代器和字符串处理。
- 文件读写与简单持久化格式。
- `match` 命令分发。
- 单元/集成测试与 `cargo test`。

运行示例：

```bash
cargo run -- add "learn ownership"
cargo run -- add "write tests"
cargo run -- list
cargo run -- done 1
cargo run -- list open
cargo run -- list done
cargo run -- clear
```

测试和格式化：

```bash
cargo fmt
cargo test
```

## 基础系列

1. [认识 Rust：语言定位、工具链与第一个程序](docs/rust-basics/01-getting-started.md)
2. [变量、不可变性与基础类型](docs/rust-basics/02-variables-and-types.md)
3. [所有权、移动与复制](docs/rust-basics/03-ownership-move-copy.md)
4. [借用、引用与切片](docs/rust-basics/04-borrowing-references-slices.md)
5. [结构体、枚举与模式匹配](docs/rust-basics/05-structs-enums-pattern-matching.md)
6. [函数、控制流与模块组织](docs/rust-basics/06-functions-control-flow-modules.md)
7. [错误处理：Option、Result 与 ? 运算符](docs/rust-basics/07-error-handling.md)
8. [集合类型、字符串与迭代器入门](docs/rust-basics/08-collections-strings-iterators.md)
9. [泛型、Trait 与生命周期基础](docs/rust-basics/09-generics-traits-lifetimes.md)
10. [Cargo、测试与实用开发工作流](docs/rust-basics/10-cargo-testing-workflow.md)

## 深入研究系列

1. [Rust 所有权模型的编译期推理机制](docs/rust-deep-dive/01-ownership-compile-time-reasoning.md)
2. [借用检查器与非词法生命周期](docs/rust-deep-dive/02-borrow-checker-and-nll.md)
3. [Trait 系统、动态分发与对象安全](docs/rust-deep-dive/03-traits-dynamic-dispatch-object-safety.md)
4. [泛型单态化、代码膨胀与性能取舍](docs/rust-deep-dive/04-generics-monomorphization-code-size.md)
5. [Rust 内存布局、Drop 顺序与 unsafe 边界](docs/rust-deep-dive/05-memory-layout-drop-unsafe-boundaries.md)
6. [async/await、Future 与执行器模型](docs/rust-deep-dive/06-async-await-future-executor.md)
7. [Send、Sync 与并发安全抽象](docs/rust-deep-dive/07-send-sync-concurrency-safety.md)
8. [宏系统：声明宏、过程宏与代码生成](docs/rust-deep-dive/08-macros-declarative-procedural-codegen.md)
9. [FFI、ABI 与跨语言边界设计](docs/rust-deep-dive/09-ffi-abi-cross-language-boundaries.md)
10. [Rust 编译器架构与 MIR/LLVM 优化管线](docs/rust-deep-dive/10-rust-compiler-mir-llvm-pipeline.md)

## 建议阅读顺序

如果是第一次系统学习 Rust，建议按基础系列编号顺序阅读。Rust 的语法并不复杂，真正需要建立直觉的是所有权、借用、生命周期、错误处理和 trait 抽象；前几篇会反复使用小例子帮助形成这些概念之间的联系。

完成基础系列后，再阅读深入研究系列。深入文章更关注 Rust 的编译期推理、运行时表示、并发约束、宏展开、FFI 边界和编译器管线，适合作为进一步研究 Rust 语言设计和工程实践的入口。
