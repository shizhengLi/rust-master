# Rust 知识文章项目

这个仓库用于系统整理 Rust 语言学习材料。当前已经完成 10 篇 Rust 基础知识介绍文档，以及 10 篇深入研究 Rust 语言机制的文章。

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
