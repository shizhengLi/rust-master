# 借用检查器与非词法生命周期

## 研究目标

- 理解借用检查器检查的不是语法风格，而是访问冲突。
- 掌握非词法生命周期如何缩短借用活跃区间。
- 知道常见借用错误背后的控制流原因。

## 借用检查器要证明什么

借用检查器的目标是证明引用使用安全。它关注三个问题：

1. 引用指向的数据是否仍然有效。
2. 同一数据是否同时存在冲突访问。
3. 可变访问是否具有独占性。

Rust 的引用规则常被记成：

- 任意数量的不可变引用。
- 或者一个可变引用。
- 两者不能同时活跃。

这不是为了限制写法，而是为了建立别名规则。不可变引用允许共享读取，可变引用要求独占修改。如果可变引用和其他引用同时存在，就可能出现读取方看到中间状态、迭代器失效或数据竞争。

## 词法生命周期的限制

早期 Rust 借用检查更接近词法作用域。只要引用变量还在作用域中，编译器就倾向于认为借用仍然存在：

```rust
fn main() {
    let mut text = String::from("hello");

    let r = &text;
    println!("{r}");

    let m = &mut text;
    m.push_str(" world");
}
```

现代 Rust 可以接受这段代码，因为 `r` 在 `println!` 后不再使用。借用的活跃时间不是到变量作用域结尾，而是到最后一次实际使用为止。

## 非词法生命周期

非词法生命周期通常称为 NLL。它让生命周期基于控制流和使用点，而不只是源代码块边界。

```rust
fn main() {
    let mut values = vec![1, 2, 3];

    let first = &values[0];
    println!("{first}");

    values.push(4);
    println!("{values:?}");
}
```

这里 `first` 的借用在打印后结束，因此后续 `push` 可以获得可变访问。没有 NLL 时，编译器可能认为 `first` 活到整个作用域结束，从而拒绝 `push`。

## 借用活跃性与最后一次使用

借用是否活跃取决于后续是否还会使用引用：

```rust
fn main() {
    let mut value = String::from("rust");

    let shared = &value;
    let mutable = &mut value; // 编译错误

    println!("{shared}");
    println!("{mutable}");
}
```

虽然 `shared` 的声明在 `mutable` 之前，但它之后还会被使用，所以在创建 `mutable` 时不可变借用仍然活跃。

修复方式是调整使用顺序：

```rust
fn main() {
    let mut value = String::from("rust");

    let shared = &value;
    println!("{shared}");

    let mutable = &mut value;
    mutable.push('!');
    println!("{mutable}");
}
```

## 两阶段借用

Rust 还支持一些更细的规则，例如两阶段借用。常见例子：

```rust
fn main() {
    let mut values = vec![1, 2, 3];
    values.push(values.len());
}
```

`push` 需要 `&mut self`，`len` 需要 `&self`。表面看这里同时有可变和不可变借用，但编译器可以把方法调用中的可变借用分成“保留”和“激活”两个阶段：先保留可变借用，计算参数时还没真正修改，参数计算完后再激活可变借用执行 `push`。

这类规则是为了让常见安全模式可用，但不应该把它理解成所有借用冲突都能自动重排。

## 借用检查与容器元素

借用检查器通常不能证明两个动态索引一定不同：

```rust
fn main() {
    let mut values = vec![1, 2, 3];

    let a = &mut values[0];
    // let b = &mut values[1]; // 编译器可能拒绝
    *a += 1;
}
```

安全写法是使用标准库提供的分割 API：

```rust
fn main() {
    let mut values = vec![1, 2, 3];
    let (left, right) = values.split_at_mut(1);

    let a = &mut left[0];
    let b = &mut right[0];

    *a += 10;
    *b += 20;
}
```

`split_at_mut` 的实现内部使用 unsafe，但它提供了安全接口，并保证两个切片不重叠。这是 Rust 的重要模式：把复杂不变量封装在小而审计明确的 unsafe 边界内。

## 借用检查与循环

循环会让借用活跃区间变复杂：

```rust
fn find_or_insert(values: &mut Vec<String>, target: &str) -> usize {
    for (index, value) in values.iter().enumerate() {
        if value == target {
            return index;
        }
    }

    values.push(target.to_string());
    values.len() - 1
}
```

这段代码通常可以通过，因为迭代产生的不可变借用在循环后结束。如果返回的是引用，问题会更复杂：

```rust
fn get_or_insert<'a>(values: &'a mut Vec<String>, target: &str) -> &'a str {
    if let Some(index) = values.iter().position(|value| value == target) {
        return &values[index];
    }

    values.push(target.to_string());
    values.last().unwrap()
}
```

这种代码牵涉返回引用和后续可变修改，可能触发更复杂的生命周期约束。工程上可以考虑返回索引、拆分函数，或调整数据结构。

## Polonius 的研究方向

Polonius 是 Rust 借用检查器的一个长期研究方向，目标是用更精确的逻辑规则描述借用关系。它关注的问题包括：

- 更精确地区分路径和贷款关系。
- 改善某些当前被保守拒绝的安全程序。
- 让借用检查规则更容易解释和维护。

日常开发不需要依赖 Polonius，但理解它能帮助我们认识到：借用检查器不是简单语法检查，而是围绕控制流、路径、别名和区域关系做静态分析。

## 工程调试方法

遇到借用错误时，不要只盯着错误行。应该问：

1. 哪个值被借用了？
2. 借用是共享还是可变？
3. 引用最后一次使用在哪里？
4. 是否尝试在借用活跃期间移动或修改所有者？
5. 是否可以缩小引用作用域、提前计算结果、返回索引而不是引用？

常见修复方式：

- 调整语句顺序，让不可变借用先结束。
- 使用内部代码块缩短引用作用域。
- 把复杂表达式拆成几步。
- 使用 `split_at_mut`、`get_mut`、`entry` 等专门 API。
- 改变函数签名，避免同时返回引用和修改容器。

## 常见误解

- NLL 不意味着生命周期消失了；它只是更精确。
- 借用检查器不执行程序，它基于静态控制流保守判断。
- `RefCell<T>` 可以把借用检查推到运行时，但违反规则会 panic。
- unsafe 可以绕过编译器检查，但不会让别名冲突变安全。

## 继续研究

- rustc-dev-guide：MIR borrow check、region inference。
- Rust RFC 2094：Non-Lexical Lifetimes。
- Rust Reference：references、place expressions、undefined behavior。
- Rustonomicon：aliasing、splitting borrows。
