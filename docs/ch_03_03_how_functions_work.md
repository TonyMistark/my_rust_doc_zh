#### 函数(Functions)

函数在Rust代码中是很普遍的。你已经看了在语言中最重要的函数之一：main函数，是很多程序的入口。你也看到了fn关键字，它允许你声明一个新的函数。

Rust代码中的函数和变量使用snake case的代码风格，所有的单词小写并用下划线隔开。这里有一个程序包含了一个函数的定义：

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

我们定义一个函数通过输入fn并在其后面跟上函数名，并加一对圆括号。花括号告诉编译器函数的主体的开始和结束。

我们通过输入函数名加一对圆括号可以调用任何函数。因为another_function是定义在程序里，它就可以被在main函数内调用。注意，我们把another_function函数定义在main函数之后；也可以定义在之前。Rust不在乎你把你的函数定义在哪里。

让我开始一个新的叫functions二进制项目，进一步探索函数。将上面的another_function例子放入src/main.rs中并运行它。你会看到如下的输出：

```powershell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.
```

main函数中的代码会按顺序执行。首先打印"Hello, world!"信息，然后调用another_function函数并打印它的信息。

##### 参数(Parameters)

我们可以定义有参数的函数，指定变量，这是一个函数的签名。当一个函数有参数(形参)时，你可以为这些参数提供具体的值(实参)。技术上讲，这些具体的值被称为参数(arguments)，但是日常交流中，人民倾向于不区分使用形参和实参来表示函数定义中的变量或者调用函数时传入的具体值。

在这个版本中，我们给another_function增加一个参数：

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

试试运行这个程序；你会得到如下输出：

```powershell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/functions`
The value of x is: 5
```

`another_function`声明了一个参数命名为x。x的类型指定为i32.。当我们给another_function传入5时，println!宏会把5放入格式化字符串中的大括号的位置。

在函数的签名中，你必须声明每个参数的类型。这是在Rust的设计中的一个深思熟虑的决定：在定义函数的时候要求类型声明意味着编译器几乎不用去代码中别的地方去指出你想表达的类型。

当定义多个参数时，用逗号隔开每个参数的声明，示例如下：

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

这个例子函数名`print_labeled_measurement`有两个参数。第一个参数是命名为value并且是一个i32。第二个参数叫做unit_label并且是char类型。函数然后打印出了包括两个值value和unit_label。

让我们试试运行这段代码。将你的functions项目的src/main.rs替换为上面的代码并cargo run运行：

```powershell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/functions`
The measurement is: 5h
```

因为我们调用函数value:5和unit_label:h，这个项目输出包括了这两个值。

##### 语句和表达式(Statements and Expressions)

函数体由一系列的语句和一个可选的结尾表达式构成。目前为止，我们提到的函数还不包含结尾表达式，不过不过你已经见过作为语句一部分的表达式。因为Rust是一门基于表达式(expression-based)的语言，这是一个需要理解的(不同于其他语言)重要区别。其他语言并没有这样的区别，所以让我们看看语句表达式有什么区别以及这些区别是如何影响函数体的。

语句(Statements)是执行一些操作但不返回值的指令。表达式(Expressions)计算产生一个值。让我们看一些例子。

实际上，我们已经使用过语句和表达式。用let关键字创建一个变量和分配一个值就是一个语句(Statement).Listing 3-1, let y = 6;是一个语句。

src/main.rs

```rust
fn main() {
    let y = 6;
}
```

Listing 3-1: A main fuction declaration containing one statement

函数定义也是一个语句；前面的整个例子本身就是一个语句。

语句不会返回值。因此，你不能把let语句复制给另外一个变量，比如厦门的例子尝试做的，会产生一个错误：

src/main.rs

```rust
fn main() {
    let x = (let y = 6);
}
```

当你运行这个程序，你会看到如下的错误：

```rust
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are experimental
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: you can write `matches!(<expr>, <pattern>)` instead of `let <pattern> = <expr>`

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  | 

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 2 previous errors; 1 warning emitted
```

let y = 6 语句不会返回值，所以x没有绑定任何东西。这与其他语言中的不同，像C和Ruby，它们的赋值语句会返回所赋的值。在这些语言中，可以这么些x = y = 6，这样x 和 y的值都是6；在Rust中不能这样些。



表达式会计算出一个值，并且你编写的大部分Rust代码是由表达式组成的。考虑一个数学运算，比如5 + 6，这是一个表达式并计算出值11.表达式可以是语句的一部分：在Listing 3-1中，语句let y = 6;中的6是一个表达式，它计算出的值是6。函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是表达式，例如：

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

这是表达式：

```rust
{
    let x = 3;
    x + 1
}
```

这一块，在这个例子中，算出4。这个值作为let语句的一部分被绑定到y上。注意x + 1没有以分号结尾，不像你目前看到的大部分代码。表达式不包括结尾的分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。在接下来探索具有返回值的函数和表达式时要谨记这一点。

##### 有返回值的函数(Functions with Return Values)

函数可以向调用它的代码返回值。我们并不对返回值命名，但是我们必须其后用箭头->声明类型。在Rust中，函数的返回值等同于函数体最后一个表达式的值。使用return关键字指定值，可以从函数中提前返回；但是大部分函数隐式的返回最后的表达式。这是一个返回值的函数的例子：

src/main.rs

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

在five函数中没有函数调用、宏、甚至没有let语句，只有一个5、这在Rust中是一个完全有效的函数。注意，也制定了函数返回值的类型，就是 -> i32。试试运行这段代码；输出如下：

```powershell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 5
```

5是函数five的返回值，这就是为什么返回值类型是i32。让我们更加详细地测试这个例子。这里有两个重要的部分：首先，let x = five();这一行表明我们使用函数的返回值初始化一个变量。因为five函数返回5，与如下的代码相同：

```rust
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

