#### Data Types

在Rust中，每个值都有一个确定的类型，它告诉Rust要指定什么类型的数据，以便知道它如何处理这些数据。我们将研究两个数据类型子集：标量(scalar)和复合(compound)。

记住，Rust是静态类型语言，意味着在编译时间它必须知道所有变量的类型。编译器通常能推断出我们想要用的变量的类型，它基于变量的值和我们如何用它。在可能有很多种类型的情况下，比如当我们把一个String用parse转换为数字类型在第二章的["Comparing the Guess to the Secret Number"](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number)部分，我们必须增加类型的注解，如：

```rust
#![allow(unused)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
}
```

如果我们在这里不添加类型注解，Rust将会显示如下错误，错误显示编译器需要我们提供更多的我们想要用什么类型的信息：

```powershell
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations` due to previous error
```

你将看到针对其他数据类型的不同类型注解。

##### 标量(Scalar Types)

一个标量(Scalar)类型代表一个单独的值。Rust有四个主要的标量类型：整型（integers）、浮点数(floating-point number)、布尔值(Booleans)和字符类型。你可能从其他语言中也认识过这些。让我们来看看他们在Rust中是如何工作的吧。

##### 整型(Integer Types)

整数，就是没有小数部分。我们已经在第二章用过整数类型了，即u32类型。这个类型声明，表明这个值应该关联为一个无符号的整数(有符号整数类型以i开头，而不是u)，并占32位的空间。Table 3-1展示了Rust内置整数类型。我们可以使用任何这些变量成员来声明一个整数值的类型。

Table 3-1:Integer Type

| Length  | Signed | unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

每个变量都可以无符号和有符号并且显式大小。有符号和无符号是指数字是否可能是负数。它就像是数字写在纸上：当有符号的情况，一个数字用+或-来表示正负；然而，当假设这个数是整数，它是安全的，不用显示符号。有符号数字以补码形式(two's complement representation)存储。



每一个有符号的变体可以储存包含从 -(2n - 1) 到 2n - 1 - 1 在内的数字，这里 n 是变体使用的位数。所以 i8 可以储存从 -(27) 到 27 - 1 在内的数字，也就是从 -128 到 127。无符号的变体可以储存从 0 到 2n - 1 的数字，所以 u8 可以储存从 0 到 28 - 1 的数字，也就是从 0 到 255。



另外，isize和usize类型依赖计算机程序运行的架构：64位架构上它们是64位，32位架构上它们是32位的。



可以使用Table 3-2中的任何一种形式编写数字字面量(integer literals)。注意，可以是多种数字类型的数字字面值允许使用类型后缀，例如57u8来指定类型，同时也允许使用 _ 作为分隔符以方便读数，例如1_000，它的值与你指定的1000相同。

Table 3-2: Integer Literals in Rust

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte(u8 only)   | b'A'        |

那么该使用哪种类型的数字呢？如果拿不定主意，Rust的默认类型通常是不错的起点，数字类型默认是i32。isize或者usize主要作为某些集合的索引。



**整数溢出**

比方说有一个u8，它可以存放从0-255的值。那么当你将它修改为256时会发生什么呢？这被称为"整数溢出(integer overflow)"，这会导致以下两种行为之一的发生。当在debug模式编译时，Rust检查这类问题并使程序panic，这个属于被Rust用来表明程序因错误而退出。第九章["Unrecoverable Errors with panic!"](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)部分讲解.

当你使用--release标签进行release编译时，Rust不检测溢出报错。而是如果发生溢出，Rust会进行一种被称为二进制补码包装(two's complement wrapping)的操作。简而言之，值256变成0，值257变成1，以此类推。程序不会报错，但是变量会有一个值，但是这个值可能不是你期待的。依赖于整数溢出包装行为被认为是一个错误。

显式地处理溢出的可能性，你可以使用这些标准库在原生数值上提供的以下方法：

- 所有模式下都可以使用wrapping_*方法进行包装，如wrapping_add
- 如果check_*方法出现溢出，则返回None值
- 用overflowing_*方法返回值和一个bool值，表示是否溢出
- 用saturating_*方法在值的最小值或者最大值处进行饱和处理。

##### 浮点型(Floating-Point Types)

Rust也有两个浮点数原生类型，就是带小数点的数字。Rust浮点数类型就是f32和f64，分别是32位和64位。默认是f64，因为在现代CPU中，和f32的速度相当，而且还能更精确。所有的浮点数都是有符号的。

举一个例子展示浮点数的操作：

src/main.rs

```rust
fn main() {
    let x = 2.0;  // f64
    let y: f32 = 3.0  // f32
}
```

浮点数是遵循IEEE-754标准。f32类型是一个单精度浮点数，而f64是双精度浮点数。

##### 数字操作

Rust支持你所期待的所有数字类型的基本数学操作：加(addition)减(subtraction)乘(multiplication)除(division)取余(remainder)。整数出发四射无数到接近的整数。下面的代码展示如何使用每个数字操作，用let声明：

src/main.rs

```rust
fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
}
```

这些语句中的每个表达式都是用一个数学运算符，并求值为单个值，然后被绑定到一个变量上。附录B(Appendix B)包含了Rust提供的所有操作的列表。

##### 布尔类型(The Boolean Type)

如其他大多数语言一样，Boolean类型在Rust中有两个可能的值：true 和 false。Boleans是一个字节大小。布尔类型在Rust中指定用bool。例如：

src/main.rs

```rust
fn main() {
    let t = true;
    let f: bool = true; // with explicit type annotation
}
```

布尔值主要用于条件判断，例如一个if表达式。我们将在"Control Flow"讲解if表达式如何工作。

##### 字符类型(The Charater Type)

Rust的char类型是语言中最原始的类型。示例如：

```rust
fn main() {
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
}
```

注意我们指定char字面量使用单引号'，而不是String字面量，是用"双引号。Rust中的char类型是4字节大小并且代表一个Unicode标量值(Unicode Scalar Value)，这意味着它可以比ASCII表示更多内容。在Rust中，拼音字母(Accented letters)，中文，日文，韩文等字符，emoji(绘文字)以及0长度的空白字符都是有效的char值。Unicode标量值从U+0000到U+D7FF和U+E000到U+10FFFF在内的值。不过，"字符(character)"不是Unicode中的一个概念，所以人直觉上的"字符(character)"可能与Rust中的char并不符合。第八章["Storing UTF-8 Encoded Text with String"](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)会讲的更加详细。

##### 复合类型(Compound Types)

复合类型可以将多个值组合成一个类型。Rust有两个原生的复合类型：元组(tuple)和数组(array)。

###### 元组

tuple是一种将多个各种各样类型的值组合到一个复合类型的通用方法。tuple具有固定长度：一旦声明，它的大小不可以增长或者收缩。

我们在圆括号中创建一系列值通过逗号隔开(comma-separated)。tuple每个位置都有一个类型，并且元祖中不同值的类型不必相同。在这个例子中，我们添加了可选的类型注释：

src/main.rs

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

变量tup绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配(pattern matching)来解构(destructure)元组值，如：

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

程序首先创建一个tuple并绑定到变量tup上。然后它用了let和一个模式将tup分成了三个不同的变量，x、y和z。这叫做解构(destructuring)，因为它将一个元组拆成了三个部分。最后，程序打印出了y的值也是6.4.

我们也可以使用点号(.)后跟值的索引来直接访问它们的值。例如：

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

这个程序创建了一个元组，x，然后用每个元素各自的索引为每个元素创建了新的变量。与大多数编程语言一样，元组的第一个元素索引为0。

没有任何值的元组()是一种特殊的类型，只有一个值，也写成()。该类型被称为单元类型(unit type)，而该值被称为单元值(unit value)。如果表达式不返回任何其他值，则会隐式返回单元值。

###### 数组(The Array Type)

另外一种包含多个值的方式是数组(array)。与元组不同，数组的每个元素的类型必须相同。Rust中的数组与其他一些语言中的数组不同，Rust中数组的长度是固定的。

我们将数组的值写在[]中，用逗号隔开:

src/main.rs

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当你想在栈(stack)而不是堆(heap)上为数据分配控件(第四章讨论栈与堆的更多内容),或者是想要确保总是固定数量的元素时，数组非常有用。但是数组并不如vector类型灵活。vector类型是标准库提供的一个允许增长和缩小长度的类似数组的集合类型。当你不确定应该使用数组还是vector的时候，那么很可能应该使用vector。第八章会详细讨论vector。

然而，数组是更有用的，当你知道元素的数量并且不需要改变的时候。例如，如果你是使用月份的名字在一个程序中，你也许用一个数组比用一个vector好，因为你知道它总是12个元素：

```rust
#![allow(unused)]
fn main() {
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
```

你可以编写一个数组的类型，使用方括号用分号隔开来设置元素的类型如：

```rust
#![allow(unused)]
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

这里，i32是每个元素的类型。在分号之后，数字5是表明这个数组有5个元素。

你也可以初始化一个数组包含相同的值，即所有的元素值都相同，通过指定初始化值，通过分号隔开，分号后面的数字就是数组的长度。如：

```rust
#![allow(unused)]
fn main() {
let a = [3; 5];
}
```

这个叫a的数组将会包含5个元素，并且每个元素的初始值为3。这和let a = [3, 3, ,3, 3, 3]是相同的。但是更简洁的方法。

###### 访问数组元素

数组是可以在堆栈上分配的已知固定大小的单个内存块。可以使用索引访问数组元素，如：

src/main.rs

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

在这个示例中，变量名first将会获得值1，因为数组索引[0]的值是1.变量名second将会获得值2，它是数组索引[1]的值。

###### 无效的数组元素访问

让我们看看发生了什么，如果你尝试访问数组的一个元素，这个元素在数组的结尾之后呢。运行如下的代码，类似于第二章猜数字游戏，从用户输入那里获取数组索引：

src/main.rs

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
```

这段代码编译成功。如果运行这段代码使用cargo run然后输入0，1，2，3，4，这个程序会打印出在数组内对应索引的值。如果你输入一个超出范围的数字，比如10，你会看到输出如：

```powershell
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

当你使用这个无效的值索引的时候，程序的结果是在运行时间的错误。程序退出并返回错误信息，并且没有运行最后的println!语句。当你试图用一个索引访问一个元素，Rust会检查你指定的这个索引是否会超过数组的长度。如果索引与数组长度相同或者更大，Rust就会死给你看。这个检查在运行时间，特别在这个例子中，因为编译器在编译完之后就不可能知道用户会输入什么。

这是一个Rust的内存安全原则示例的表现。在很多底层代码语言中，当你提供一个错误的索引，这种类型的检查它们是不做的，无效内存就会被访问，会导致你不知道你到底访问到了别的什么奇奇怪怪的值。Rust项目中会立即退出，而不是允许你继续访问，从而保护你面授此类错误的影响。第九章将会讨论Rust的错误处理。

