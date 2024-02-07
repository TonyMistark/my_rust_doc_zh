---
title: Ch 06.01:The Option Enum and Its Advantages Over Null Values
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
### The Option Enum and Its Advantages Over Null Values

这一部分探索学习`Option`，另外一个标准库的枚举定义。`Option`类型编码了非常常见的场景，其中可以是某个值，也可以都不是。

例如，如果请求一个包含项的列表的第一个值，会得到一个值，如果请求一个空的列表，就什么也不会得到。从类型系统的角度来表达这个概念意思就是编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他语言中非常常见的bug.

编程语言的设计通常考虑的是包含哪些特性，但是你排除的特性也很重要。Rust没有许多其他语言所具有的null特性。`Null`是一个值，表示什么也没有。在带有null特性的语言中，变量总是可以处于两种状态之一：null 或 not-null.

Null的发明者Tony Hoare在他2009年的报告"Null Reference: The Billon Dollar Mistake"中这样说：

> 我称之为十亿美元的错误。当时，我在为一个面向对象语言设计第一个综合性的面向引用的类型系统。我的目标是通过编译器的自动检查来保证所引用的使用都应该是绝对安全的。不过我未能抵抗住引入一个空引用的诱惑，仅仅是因为它是这么的容易实现。这引发了无数错误，漏洞和系统崩溃，在之后的四十多年中造成了十亿美元的苦痛和伤害。

空值的问题在于，如果你试图将空值用作非空值，会出现某种形式的错误。因为空和非空的属性到处都是，非常容易出现这类错误。

然而，null试图表达的概念仍然是有用的：null是当前由于某种原因无效或者不存在的值。

问题不在于概念，而在于具体的实现。因此，Rust没有空值，但它有一个枚举，可以编码值存在或者不存在的概念。这个枚举就是`Option<T>`，它由标准库定义如下：

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>`枚举非常有用，以至于它被包含在relude中；你不需要将它显式地引入作用域。另外，它的成员也是如此（被包含在prelude中），可以不需要`Option::`前缀来使用`Some`和`None`。即便如此`Option<T>`也仍是常规的枚举，`Some<T>`和`None`仍是`Option<T>`的成员。

`<T>`语法是我们还没讲到的Rust特性。它是一个泛型参数，在Chapter 10我们将会详细讲解。目前，你需要知道的就是`<T>`意味着`Option`枚举的`Some`成员可以包含任意类型的数据，同事每一个用于`T`位置的具体类型使得`Option<T>`整体作为不同的类型。这里是一些包含数字类型和字符串类型`Option`值的例子：

```rust
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;
}
```

`some_number`的类型是`Option<i32>`。`some_char`的类型是`Option<char>`，是不同的类型。因为我们在`Some`成员中指定了值，Rust可以推断出其类型。对于`absent_number`，Rust需要我们指定`Option`整体的类型，因为编译器只通过`None`值无法推断出`Some`成员保存的值的类型。这里我们告诉Rust我们需要`absent_number`的类型是`Option<i32>`。

当我们有一个`Some`值，我们就知道存在一个值，这个值保存在`Some`中。当我们有一个值`None`，在某种意义上，它跟空值是有相同的意义：并没有一个有效的值。那么`Option<T>`为什么就比Null要好呢？

简单来说，因为`Option<T>`和`T`（`T`可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用`Option<T>`。例如：这段代码不能编译，因为它尝试将`Option<i8>`与`i8`相加：

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    let sum = x + y;
}
```

如果我们运行这段代码，我们会得到一个错误信息如下：

```
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <&'a f32 as Add<f32>>
            <&'a f64 as Add<f64>>
            <&'a i128 as Add<i128>>
            <&'a i16 as Add<i16>>
            <&'a i32 as Add<i32>>
            <&'a i64 as Add<i64>>
            <&'a i8 as Add<i8>>
            <&'a isize as Add<isize>>
          and 48 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` due to previous error
```

铛!铛!铛!事实上，这个错误意味着Rust不知道如何将`i8`和`Option<i8>`相加，因为它们是不同类型。在Rust中，当我们有一个值像`i8`，编译器将会确保我们总是有一个有效的值。我们可以自信地使用而无需做空检查。只有当使用`Option<i8>`(或者任何用到的类型)我们需要担心可能没有一个有效值，并且编译器将会确保我们在使用值之前处理了空的情况。

换句话说，你必须将`Option<T>`转换成`T`之后才能对`T`进行运算。通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空，但实际上是空的情况。

消除错误地假设非空值的风险有助于你对代码更有信心。为了有一个可能为空的值，你必须显式地使用`Option<T>`。然后，在使用该值时，需要显式地处理值为空的情况。任何地方，一个值不是`Option<T>`类型，你都可以放心地假设他不是null。这是Rust有意为之的设计决定，目的就是限制null的滥用，以增加Rust代码的安全性。

所以，当你有一个类型为`Option<T>`的值时，你如何从`Some`变量中获得`T`的值呢？`Option<T>`枚举有大量很有用的方法适用于各种情况；你可以查看它的文档(https://doc.rust-lang.org/std/option/enum.Option.html)。熟悉`Option<T>`上的方法将对你的Rust之旅很有用哦！

通常，为了使用`Option<T>`值，你需要有处理每个变量的代码。你希望一些代码只在具有`Some<T>`值时运行，并且允许这些代码使用内部的`T`。你希望如果有个`None`值时运行一些其他的代码，而且这些代码也没有一个`T`的有效值。`match`表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。

