---
title: Ch 03.01:隐藏(Shadowing)
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
##### 隐藏(Shadowing)

如你在第二章猜数字游戏教程所见，你可以声明一个新的变量用和前面声明的变量同样的名字。Rustaceans说第一个变量被第二个隐藏(Shadowing)了，意思是第二个变量的值是在程序使用时才看到的。我们可以shadow一个变量使用相同的变量名，并重复地使用let关键字，如下：

src/main.rs

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

程序首先绑定了5给x。然后重新用let x=隐藏了x，使得原来的值加1，所以变量的值变为了6。然后，在内部作用域，第三次let声明x, 将之前的值乘以2得到12。当内部作用结束，内部的隐藏结束，并且x变回6。当我们运行这个程序，它运行结果如下：

```powershell
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

隐藏不同于将变量标记为mut，因为如果我们意外地尝试重新分配这个变量不使用let关键字，我们会得到一个编译错误。通过使用let，我们可以对一个值执行一些转换，但在这些转换完成后，变量是不可变的。

其他的mut和shadowing之间的不同之处是因为当我们再次使用let关键字，我们实际上创建了一个新的变量。例如，假如我们的程序要求用户输入空格来显示文本之间需要多少空格，然后我们希望将输入存储为数字：

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

首先spaces变量是一个string类型，然后第二个spaces变量是数字类型。因此shadowing使我们不需使用不同的变量名，如space_str 和 space_num；然而，如果我们尝试使用mut，如下，我们会得到一个编译时错误：

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
}
```

错误说我们不被允许转变变量的类型：

```rust
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

现在，我们探索了变量是如何工作的，我们还有更多的数据类型等着你呢！