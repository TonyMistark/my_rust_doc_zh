#### Variables and Mutability

如之前[Storing Values with Variables](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#storing-values-with-variables)部分提到的，默认情况变量是不可变的(immutable)。这也是众多Rust提供的众多优势之一，让你得以充分利用Rust提供的安全性和简单并发性来编写代码。然而，你仍然有选项可以是你的变量可变(mutable)。让我们来探索一下Rust为何以及如何鼓励你利用不可变性，以及何时你会选择不使用不可变性。



当变量不可变时，一旦被绑定到这个变量名，你不能修改这个变量的值。为了说明这个，让我们生成一个叫variables的项目，使用：cargo new variables

然后，在你新建的variables目录，打开src/main.rs用如下代码并替换里面的代码。这些代码还不能编译，你将第一次检测不可变(immutability)错误。

src/main.rs

```rust
fn main() {
  let x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x;)
}
```

保存并运行程序cargo run。你应该会收到一个错误信息，如下：

```powershell
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

这个例子显示编译器如何帮你在程序中找到错误。编译器错误可能让人很烦，但是的确它代表你的程序想做的事情是不安全的。它们不是代表你不是一个好的程序设计师！有经验的Rustaceans同样也会有编译错误。



这个错误表明，造成这个错误的是你不能两次给不可变的变量x分配值(cannot assign twice to immutable variable `x`)，因为你尝试分配两个值给不可变的变量x。



我们获得编译时的错误是很重要的，当我们试图修改一个指定为不可变的变量的值，因为这是非常常见的容易导致bug的情况。假设我们有一段代码设置了一个值将永远不会改变，但是另外一段代码改变了这个值，这可能会导致第一段代码变得不是它本来想要的那种设计所想做的事了。事实上，这会导致各种各样的bugs，而且很难定位，尤其当第二段代码只在某种情况下修改这个值。Rust编译器保证当你声明一个变量不可以修改，它就是不可以修改，所以你不用自己去保持追踪它。因此，你的代码更容易推理。



但是可变的变量也是非常有用的，而且让代码编写起来更加便利。变量只是在默认情况下是不可变的；如你在第二章所做的那样，你可以通过在编码名前加上mut将它设置为可变的(mutable)。添加mut也能传达给读者你的目的，通过它表明另外一段代码可能会修改这个变量的值。

例如，让我修改src/main.rs代码如下：

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

当我们运行程序，会得到：

```powershell
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

当使用了mut,我们允许修改变量绑定的值从5改为6。为了防止bug外，还需要多重权衡考虑。例如，在使用大数据结构的情况，与复制并返回一个新的分配的实例相比，就地修改一个值可能更快。对于小的数据结构，用函数式风格创建实例和编写代码可能更容易思考。所以为了更加清晰，轻微的低效也许是值得的。

##### 常量(Constants)

像不可变量，常量是绑定名称且不可更改的值，但是这里有一些些不同之处。

首先，你不被允许对常量使用mut。常量是默认不可变--它们永远不可变。声明常量使用const关键字而不是let关键字，并且必须对值的类型进行注解。我们将在下一节讲"Data types"，所以现在不用担心这些细节。现在你知道了必须总是要注解类型。

常量可以在任何作用域声明，包括全局作用域，它对一些值在很多地方要被知道是很有用的。

最后一个不同是常量只能设置为常量表达式，而不是只能在运行时在计算值。

这里有一个常量声明的示例：

```rust
fn main() {
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

这个常量的名字是THREE_HOURS_IN_SECONDS 并且它的值设置为3小时。Rust的常量命名惯例是所有单词大写用下划线连接。编译器能够在编译时计算一组有限的操作，这样编写值的方式更容易理解和验证。而不是直接设置常量为10800。[RustReference's section on constant evalation](https://doc.rust-lang.org/reference/const_eval.html)会有多详细的信息。

常量在整个运行时间在它的作用域内部都是有效的。这个属性使得常量对于应用程序域中很有用，因为程序的多个部分可能都需要这些值。比如在游戏中任何玩家的最大点数或者光速。

将遍布于应用程序中的硬编码声明为常量，能帮助未来维护代码的人员了解值的意图。如果将来需要修改硬编码，也只需要修改汇聚于一处的硬编码。

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