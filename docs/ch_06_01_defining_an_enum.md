## 定义一个枚举(Defing an Enum)

结构体给你一个组织相关联字段和数据的方式，像`Rectangle`和它的`width`和`height`，而枚举是给你另外一种方式来表达某一个值是某些值的集合之一。例如，矩形(Rectangle)是一些个形状的集合之一，这个集合还可能包括了圆(Circal)和三角形(Triangle)。因此，Rust允许我们将这些可能的值编码成为一个枚举。

让我们来看看我们可能要用代码来表达的一种可能，并看看为什么枚举是如此的有用，而且在这个场景下枚举比结构体更加贴切。假设我们需要用到IP地址。现在，有两个主要的标准用于IP地址：ipv4和ipv6。因为IP地址只会遇到这两种可能，我们可以枚举所有的可能的变量，这也是枚举取名的地方。

任何IP地址不是ipv4就是ipv6,但是不会同时出现。这就是IP地址使得枚举比结构体更加贴切的原因。因为ipv4和ipv6本质都是IP地址，所以当代码处理适用于任何类型的IP地址的情况时，他们应该被当做同一种类型对待。

我们可以通过在代码中定义一个`IpAddrKind`枚举来表现这个概念并列出可能的IP地址类型，V4和V6。如下就是枚举变量的定义：

```rust
enum IpAddrKind {
    V4,
    V6
}
```

现在`IpAddrKind`就是一个自定义数据类型， 我们可以在我们的代码的任何地方使用。

### 枚举变量(Enum Values)

我们可以创建`IpAddrKind`的两个变量的实例， 如下所示：

```rust
let four = IpAddrKing::V4;
let six = IpAddrKing::V6;
```

注意，枚举的成员位于其标识符的命名空间中，并使用两个分号隔开。这么设计的好处是现在`IpAddrKing::V4`和`IpAddrKind::V6`都是`IpAddrKind`类型的。例如，接着可以定义一个函数来获取任何IpAddrKind:

```rust
fn route(ip_kind: IpAddrKind) {}
```

使用枚举甚至更加有优势。进一步思考一下我们的IP地址类型，目前我们没有实际存储IP地址的数据；我们只知道是那种类型。鉴于刚刚Chapter 5已经学习了结构体，你可能会像List6-1那样处理这个问题：

```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

Listing 6-1: Storing the data and `IpAddrKing`variant of an IP address using a `struct`

这里，我们已经定义了一个结构体`IpAddr`，它又两个字段：一个是`IpAddrKind`类型的`kind`字段，一个是`String`类型的`address`字段。我们有这个结构体的两个实例。第一个是`home`，并且它有一个值为`IpAddrKind::V4`的`kind`和`address`值为`127.0.0.01`。第二个实例是`loopback`。它有另外一个`IpAddrKind`类型值为`V6`的`kind`和值为`::1`的`address`关联在一起。我们使用结构体绑定了`kind`和`address` ，所以现在变量和值联系在了一起。

然而，代表同样的概念的使用一个枚举更加简洁：仅仅使用枚举将数据放进每一个枚举成员而不是将枚举作为结构体的一部分。`IpAddr`枚举的定义表明了`V4`和`V6`成员都关联了`String`值:

```rust
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

我们已经展示了几种不同的方式来定义数据结构，来存储IPv4和IPv6。然而，事实证明，想要存储IP地址并对它们的类型进行编码是非常普遍的，并且标准库已经定义了，可以直接使用(https://doc.rust-lang.org/std/net/enum.IpAddr.html) 让我们看看标准库是怎么定义`IpAddr`：它具有我们已经定义和使用的确切的枚举和变量，但是它以两种不同结构的形式将地址数据嵌入到变量中，这两种结构对每个变量定义不同：

```rust
#![allow(unused)]
fn main() {
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
}
```

这段代码描绘了你可以在枚举变量中放入任何类型的数据：例如，字符串，数字类型或者结构体。你甚至可以包含另一个枚举！此外，标准库通常不会比你想到的复杂多少。

注意，尽管标准库包含`IpAddr`的定义，我们仍然可以创建和使用我们自己的定义而不会发生冲突，因为我们还没有将标准库的定义引入到我们的作用域。我们将在Chapter 7中详细讨论如何将类型引入作用域。

让我们看一下Listing 6-2的另外一个例子：这个枚举的变体中嵌入了各种各样的类型。

```rust
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {}
```

Listing 6-2: A `Message`enum whose variants each store different amounts and types of values

这个枚举有四个不同类型的变量：

* `Quit`没有与之关联的数据
* `Move`像一个结构体
* `Write`包含一个字符串`String`
* `ChangColor`包含三个`i32`值

定义带有Listing6-2中所示变量的枚举，类似于定于不同类型的结构体,除了枚举不适用关键词`struct`,并且所有的变量都被分组再`Message`类型下。下面的结构可以与前面的枚举变量保存相同的数据。

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String);  // tuple struct
struct ChangeColorMessage(i32, i32, i32);  // tuple struct

fn main() {}
```

但是如果我们使用不同的结构体（每个结构体都有自己的类型），就不能像使用List6-2定义的`Message`枚举那样轻松地定义函数来接受任何类型的消息。

枚举和结构体还有一个相似之处：正如我们能够使用`impl`在结构体上定义方法一样，我们也能够在枚举上定义方法。这是一个名为`call`的方法，我们可以在`Message`枚举上定义它：

```rust
fn main() {
    enum Message {
      Quit, 
      Move {x: i32, y: i32},
      Write(String),
      ChangeColor(i32, i32, i32),
  }
  impl Message {
      fn call(&self) {
        // method body wold be defined here
    }
  }
  let m = Message::Write(String::from("hello"));
  m.call();
}
```

方法的主体会使用self来获取我们调用方法的值。在本例中，我们创建了一个变量`m`，其值为`Message::Write(String::from("hello"))`，这就是当`m.call()`运行时`self`在调用方法主体中的位置。



让我们看看标准库中另外一个非常常见和有用的枚举：`Option`



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

