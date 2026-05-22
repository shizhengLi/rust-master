# Rust 知识文章项目

这个仓库用于系统整理 Rust 语言学习材料。当前阶段已经完成 10 篇 Rust 基础知识介绍文档，后续可以继续补充 10 篇深入研究 Rust 语言机制的文章。

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

## 后续深入系列选题

1. Rust 所有权模型的编译期推理机制
2. 借用检查器与非词法生命周期
3. Trait 系统、动态分发与对象安全
4. 泛型单态化、代码膨胀与性能取舍
5. Rust 内存布局、Drop 顺序与 unsafe 边界
6. async/await、Future 与执行器模型
7. Send、Sync 与并发安全抽象
8. 宏系统：声明宏、过程宏与代码生成
9. FFI、ABI 与跨语言边界设计
10. Rust 编译器架构与 MIR/LLVM 优化管线

## 建议阅读顺序

如果是第一次系统学习 Rust，建议按基础系列编号顺序阅读。Rust 的语法并不复杂，真正需要建立直觉的是所有权、借用、生命周期、错误处理和 trait 抽象；前几篇会反复使用小例子帮助形成这些概念之间的联系。
