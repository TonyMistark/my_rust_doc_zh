---
title: Ch 05.02:[derive(Debug)]
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

我们可以把`dbg!`放在表达式放在`30 * scale`周围，以为你`dbg!`返回表达式的值的所有权，所以width字段将获得相同的值，就像我们在这里没有`dbg!`调用一样。我们不希望dbg!拥有rect1的所有权，所以我们在下一次调用dbg!时传递一个引用。下面是这个例子的输出结果：

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

我们可以看到第一条输出来自src/main.rs第10行，我们正在调用表达式`30 * scale`，其结果值是60(为整数实现的Debug格式化是只打印它们的值)。在src/main.rs第14行的dbg!调用输出&rect1的值，即`Rectangle`类型。`dbg!`宏确实很有用。

除了`Debug`trait，Rust还为我们提供了很多通过derive属性来使用的trait，它们可以为我们的自定义类型增加实用的行为。这些trait和行为被列举在附录C(Appendix C)。第十章会介绍如何通过自定义行为来实现这些trait，同时还有如何创建你自己的trait。除了derive之外，还有很多属性；更多信息见"Rust Reference"的Attribute部分。

我们的`area`函数是非常特殊的，它只计算长方形的面积。如果这个行为与`Rectangle`结构体再结合得更紧密一些就更好了，因为它不能用于其他类型。现在让我们看看如何继续重构这些代码，来将`area`函数协调进`Rectangle`类型定义的`area`方法中。


















