#### An Example Program Using Structs

为了理解何时需要使用结构体，让我们编写一个计算矩形面积的程序。我们将通过使用单个变量开始，然后重构这个程序，直到用结构体替代它。

让我们用cargo创建一个新的项目叫做`rectangles`的二进制程序，它获取以像素为单位的矩形的宽度和高度，并计算出矩形的面积。Listing 5-8 展示一个简短的程序：

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

Listing 5-8: Calculating the area of a rectangle specified by separate width and height variables

现在，运行这个程序，通过 cargo run:

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
The area of the rectangle is 1500 square pixels.
```

这段代码通过调用area函数成功地计算出矩形的面积，但是我们可以做的更多使得代码看上去简洁可读。

这些代码的问题突显在area函数签名上:

```rust
fn arean(width: u32, height: u32) -> u32 {}
```

这个area函数是假定计算一个矩形的面积，但是我们写的函数有两个参数，并且这两个参数是相关联的，不过程序本身却没有表现出来这一点。它可以将width和height组合起来使得更加有可读性。第三章"The Type Type"已经讨论过通过tuple这种可行性了。

#### 用元组重构(Refactoring with Tuple)

Listing 5-9 展示了另外一个用tuples实现的版本。

filename: src/main.rs

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

Listing 5-9: Specifying the width and height of the reactangle with a tuple

一方面，这个程序更好。tuple让我们添加了一点数据结构，并且，我们现在只传一个参数。但是另外一方面，这个版本缺乏清晰感：tuple没有对它的元素命名，所以我们必须通过索引来计算，使得我们的计算看上去不清晰明了。

混合width和height对计算无关紧要，但是如果我们想在屏幕上画出这个矩形，它就是紧要问题！我们必须时刻记着width的索引是0，height的索引是1。因为我们的代码没有传递出数据的这个额信息，它就会更容易引发错误。

#### 用结构体重构：增加更多信息(Refactoring with Structs: Adding More Meaning)

我们使用结构体通过数据的标签(labeling)来赋予意义。我们可以变tuple为结构体，并对每一个元素命名，如Listing: 5-10所示：

Filename: src/main.rs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

这里我们定义了一个结构体并命名为Reactangle。在花括号中，我们定义了width和height两个字段。两个字段类型都是u32。然后在main中，我们创建了一个Reactangle的特殊实例，并设置width 30,height 50。

我们的area函数现在是定义了一个参数，这个参数命名为rectangle，类型是一个不可变的借用了一个Rectangle的实例。如第四章所提及的，我们希望借用结构体而不是获取它的所有权，这样main函数就可以保持rect1的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有&。

area函数访问Rectangle实力的width和height字段。area的函数签名现在明确的阐述了我们的意图：使用Rectangle的width和height字段，计算Rectangle的面积。这表明宽和高是有联系的，并为这些值提供了描述性的名称而不是索引的值0和1.结构体胜在更加清晰明了。

#### 通过派生trait增加实用功能(Adding Useful Functionality with Derived Traits)

在调试程序时打印出Rectangle实例来查看其所有字段的值是很有用的。Listing 5-11 像前面章节一样尝试使用print 宏。但这并不行。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

Listing 5-11: Attempting to print a Rectangle instance

当我们编译这段代码，我们会得到一个错误以及核心信息：

```rust
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!`宏可以做很多的格式化，默认情况下，通过花括号告诉`println`用被称为Display的格式：意在提供给终端用户查看输出。目前为止见过的类型都默认实现了Display，因为它是向用户展示1或任何其他的原始类型。但是结构体，因为结构体显示的可能性太多，而应该格式化的输出是不够清晰的：是否需要逗号？是否需要打印大括号？所有的字段都要显示吗？由于这种不确定性，Rust不会尝试猜测我们的意图，所以结构体并没有提供一个Display实现来使用`println!`与{}占位符。

但是如果我们继续阅读错误，将会发现这个有用的信息：

```rust
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

让我们尝试一下吧！`println`宏调用看起来像`println!("rect1 is {:?}", rect1);`这样。在{}中加入`:?`指示符告诉`println!`我们想要使用叫做`Debug`的输出格式。Debug是一个`trait`，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值。

这样调整后再运行程序。Oh shiiiiiiit,它咋还报错呢：

```rust
error[E0277]: `Rectangle` doesn't implement `Debug`
```

但是再次，编译器给了我们有用的帮助信息:

```rust
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust确实包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能。为此，在接头体定义之前加上外部属性`#[derive(Debug)]`，如Listing 5-12 所展示：

Filename: src/main.rs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

Listing 5-12: Adding the attribute to derive the `Debug`trait adn printing the `Rectangle`instance using debug formatging

现在，当我们运行程序，我们就不会看到有任何报错了，并会看到如下输出：

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle { width: 30, height: 50 }
```

Nice！这并不是漂亮的输出，不过它显示这个实例的所有字段，毫无疑问这对调试有帮助。当我们有一个更大的结构体时，能有更易读一点的输出就好了，为此可以使用`{:#?}`替换`println!`中的`{:?}`。在这个例子中使用`{:#?}`风格将会输出：

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

另一种使用`Debug`格式打印数值的方法是使用`dbg!`宏，它接收一个表达式的所有权，打印出代码中调用dbg!宏时所在的文件和行号，以及表达式的结果，并返回该值的所有权。

```
注意：调用dbg!宏会打印到标准错误控制台流(stderr)，与println!不同，后者会打印到标准输出控制台(stdout)。我们将在第十二章"Writing Error Message to Standard Error Instead of Standard Output"讨论。
```

这里有一个例子：我们关心于分配给width字段的值以及rect1中整个结构的值。

```rust
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


















