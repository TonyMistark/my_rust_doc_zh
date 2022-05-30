#### 注释(Comments)

所有的程序设计人员努力使他们的代码更容易理解，但是有时候额外的解释是有必要的。在这种情况下程序设计人员在源代码中留下注释(comments)，编译器会忽略这些注释，但是人阅读源码会发现它很有用。

这里有一个简单的注释例子：

```rust
#![allow(unused)]
fn main() {
// hello, world
}
```

在Rust中，惯用的注释风格是以两个斜杠(slashes)开始，并且注释一直到行位。对于超过一行的注释，你将需要让每行都包括//。示例如下：

```rust
#![allow(unused)]
fn main() {
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
}
```

注释可以放在代码行结尾：

src/main.rs

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today
}
```

但是你经常看到更多是如下的格式，在代码的上方的单独一行注释：

```rust
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
```

Rust 也有其他类型的注释，文档注释(documentation comments)，详细在第14章的"Publishing a Crate to Crates.io" 部分介绍。