---
title: Ch 03.03:![allow(unused)]
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
#### 函数(Functions)

函数在Rust代码中是很普遍的。你已经看了在语言中最重要的函数之一：main函数，是很多程序的入口。你也看到了fn关键字，它允许你声明一个新的函数。

Rust代码中的函数和变量使用snake case的代码风格，所有的单词小写并用下划线隔开。这里有一个程序包含了一个函数的定义：

```
#![allow(unused)]
fn main() {
let x = 5;
}
```

其次，five函数没有参数并定义了返回值类型，不过函数体只有单单一个5，因为它是一个表达式，可以返回我们想要的值。

让我们看看另外一个例子：

src/main.rs

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

编译这段代码，会产生一个如下的错误：

```powershell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: consider removing this semicolon

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

主要的错误信息"mismatched types"揭示了这段代码的核心问题所在。定义函数plus_one，然后说要返回一个i32，但是语句不会计算得到一个值，使用单位类型()表示不返回值。因为不返回值与函数返回一个i32类型的值矛盾，从而出现一个错误。在输出中，Rust提供了一条信息，可能有助于纠正这个错误：它建议删除分号，这会修复这个错误。

正确的代码如下：

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

