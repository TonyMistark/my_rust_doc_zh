## 使用use关键字将路径引入作用域

必须编写路径来调用函数显得不方便并且重复。在 Listing 7-7，选择`add_to_waitlit`函数的绝对路径还是相对路径，每次我们想要调用`add_to_waitlist`时，都必须指定`front_of_house`和`hosting`。幸运的是，有一种方法可以简化这个过程。我们可以使用`use`关键字创建一个短路径，然后就可以在作用域中的任何地方使用这个更短的名字。

在Listing 7-11中，我们将`crate::front_of_house::hosting`模块引入了`eat_at_restaurant`函数的作用域，而我们只需要指定`hosting::add_to_waitlist`即可在`eat_at_restaurant`中调用`add_to_waitlist`函数。

src/lib.rs

```rust
mod front_of_hose {
    pub mod hosting {
      pub fn add_to_waitlist() {}
  }	
}

use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
  	hosting::add_to_waitlist();
}
```

Listing 7-11: Bring a module into scope with `use`

在作用域中用`use`和路径类似于在文件系统中创建软链接(symbolic lick)。通过在crate根添加`use crate::from_of_hose::hosting`，`hosting`在当前作用域是一个有效的名字，就像`hosting`模块被定义于crate根一样。通过`use`引入作用域的路径也会检查私有性，同其他路径一样。

注意`use`只能创建`use`所在的特定作用域内的短路径，Listing 7-2将`eat_at_restaurant`函数移动到了一个叫`customer`的子模块，这又是一个不同于`use`语句的作用域，所以函数体不能编译。

src/lib.rs

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house::hosting;

mod customer {
  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
  }
}
```

Listing 7-12: A `use` statement only applies in the scope it's in

编译器错误显示短路径不在使用于`customer`模块中:

```rust
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` due to previous error; 1 warning emitted
```

注意这里还有一个警告说`use`在其作用域内被使用！为了修复这个问题，可以将`use`移动到`customer`模块内，或者在子模块`customer`内通过`super::hosting`引用父模块中的短路径。

#### 创建惯用的`use`路径(Creating Idiomatic use Paths)

在Listing 7-11中，你也许有疑问，为什么我们指定`use crate::front_of_house::hosting`并且在`eat_at_restaurant`内调用`hosting::add_to_waitlist`，而不是通过指定一直到`add_to_waitlist`函数的`use`路径来得到相同的结果，如Listing 7-13所示。

src/lib.rs

```rust
mod front_of_house {
		pub mod hosting {
			pub fn add_to_waitlist() {}
		}
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
		add_to_waitlist();
}
```

Listing 7-13:Bringing the `add_to_waitlist` function into scope with `use`，which is unidiomatic

虽然Listing 7-11和7-13都完成了相同的任务，但Listing 7-11是使用`use`将函数引入作用域的习惯用法。要想使用`use`将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。Listing 7-13中的代码不清楚`add_to_waitlist`是在哪里被定义的。

另一方面，使用`use`引入结构体，枚举和其他项时，习惯是指定它们的完整路径。Listing 7-14将`HashMap`结构体引入二进制crate作用域的习惯用法。

src/main.rs

```rust
use std::collections:HashMap;

fn main() {
  	let mut map = HashMap::new();
  map.insert(1, 2);
}
```

Listing 7-14: Bringing `HashMap` into scope in an idiomatic way

这里没有什么特别强制性的要求：这只是一种惯例，人们已经习惯了这样阅读和编写Rust代码。

这个习惯有一个例外，那就是我们使用`use`语句将两个具有相同名称的项带入作用域，因为Rust不允许这样做。Listing 7-15展示了如何将两个具有相同名称但不同父模块的`Result`类型引入作用域，以及如何引用他们。

src/lib.rs

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
  Ok(())
}

fn function2() -> io::Result<()> {
  Ok(())
}
```

Listing 7-15: Bringing two types with the same name into the same scope requires using their parent modules.

如上面的示例所示使用父模块区别这两个`Result`类型，如果我们指定`use std::fmt::Result`和`use std::io::Result`，在一个作用域有两个`Result`类型，我们就没办法区别这两个`Restult`类型了

#### 使用`as`关键字提供新的名称(Providing New Names with the `as` Keyword)

使用`use`将两个同名类型引入同一作用域还有另外一个解决办法：在这个类型的路径后面，我们使用`as`指定一个新的本地名称或者别名。Listing 7-16展示了另一个编写Listing 7-15中代码的方法，通过`as`重命名其中一个`Result`类型。

src/lib.rs

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
  Ok(())
}

fn function2() -> IoResult {
  Ok(())
}
```

Listing 7-16: Renaming a type when it's brought into scope with the `as` keyword

在第二个`use`声明中，我们为`std::io::Result`类型选择一个新名字`IoResult`，它就不会和`std::fmt`的`Result`冲突了。Listing 7-15和Listing7-16被认为是惯例，所以用不用取决于你。

#### 使用`pub use`重新导出名称(Re-exporting Names with `pub use`)

当我们用`use`关键字引入一个名称到作用域中，这个引入的可用的名字在新的作用域中是私有的。如果想让其他人调用我们的代码时，也能够正常使用这个名称，就像它本来就在当前作用域一样，那我们可以将`pub`和`use`结合起来使用。这种技术被称为重导入(re-expeorting)：我们不仅将一个名称导入到当前作用域，还允许别人使用它导入他们自己的作用域。

Listing 7-17将Listing7-11根模块中的`use`改为`pub use`。

src/lib.rs

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}
```

Listing 7-17: Making a name avaliable for any code to use from a new scope with `pub use`

在这个修改之前，外部代码还需要使用路径`restaurant::front_of_house::hosting::add_to_waitlist()`来调用`add_to_waitlist`函数。现在这个`pub use`从根模块重导出了`hosting`模块，外部代码现在可以使用路径`restaurant::hosting::add_to_waitlist`。

当你代码的内部结构与调用你代码的程序所想象的结构不同时，重导出会很有用。例如，在这个餐馆的比喻中，经营餐馆的人会想到"前台"和"后台"。但顾客光顾一家餐馆时，可能不会以这些术语来考虑餐馆的各个部分。使用`pub use`，我们可以使用一种结构编写代码，却将不同的结构形式暴露出来。这样做使我们的库井井有条，也使开发这个库的程序员和调用这个库的程序员都更加方便。我们还会在第十四章"Exporting a Convenient Public API with `pub use`"看到另外一个例子以及它如何影响你的crate的文档。

#### 使用外部包(Using External Packages)

在第二章中，我们编写了一个猜数字的小项目，调用了一个外部的包叫做`rand`来生成随机数。为了在项目中使用`rand`我们在Cargo.toml文件添加了如下一行配置:

Cargo.toml

```toml
rand = "0.8.5"
```

在Cargo.toml中加入`rand`以来来告诉Cargo要从crate.io下载`rand`以及它的依赖，并使其可在项目代码中使用。

接着，为了将`rand`引入项目包的作用域，我们加入一行`use`起始的包名，它以`rand`包名开头并列出了需要引入作用域的项。回忆一下第二章的"生成一个随机数"的部分，我们将`Rng` trait引入作用域并调用了`rand::thread_rng`函数：

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

crate.io 上有很多Rust社区成员发布的包，将其引入你自己的项目需要一道相同的步骤：在Cargo.toml列出它们并通过`use`将其中定义的项的引入项目包的作用域中来引用它们，比如我们使用`HashMap`：

```rust
use std::collections::HashMap;
```

这是一个以标准库crate名`std`开头的绝对路径。

#### 嵌套路径来消除大量的use行(Using Nested Paths to Clean Up Large use Lists)

如果我们需要引入很多定义于相同模块的项时，为每一项单独列出一行会占用源码很大的空间。例如第二章的猜数字项目Listing 2-4中有两个`use`语句都从`std`引入项到作用域：

src/main.rs

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

相反我们可以使用嵌套路径将相同的项在一行中引入作用域。这么做需要指定路径的相同部分，接着是两个冒号，接着是大括号中的各自不同的路径部分，如Listing 7-18所示：

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

Listing 7-18: Specifying a nested path to bring multiple items with the same prefix into scope

在大型项目中，使用嵌套路径从相同包或者模块中引入很多项，可以显著减少所需的独立`use`语句的数量！

我们可以在路径的任何层级使用嵌套路径，这在组合两个共享子路径的`use`语句时非常有用。例如，Listing 7-19中展示了两个`use`语句：一个将`std::io`引入作用域，另一个将`std::io::Write`引入作用域

src/lib.rs

```rust
use std::io;
use std::io::Write;
```

Listing 7-19: Two `use`statements where one is a subpath of the other

两个路径的相同部分`std::io`，这正是第一个路径。为了在一行`use`语句中引入两个路径，可以在嵌套路径中使用`self`，如Listing 7-20.

src/lib.rs

```rust
use std::io::{self, Write};
```

Listing 7-20： Combining the paths in Listing 7-19 into one `use` statement

这行将`std::io`和`std::io::Write`都引入了作用域。

#### 全局操作符(The Glob Operator)

如果想将一个路径下所有公有项引入作用域，可以使用指定路径后跟`*`，glob操作符:

```rust
use std::collections::*;
```

这个`use`语句会将`std::collections`中定义的所有的公有项引入到当前作用域。使用全局操作符时一定要多加小心！全局操作符会使得我们难以推导作用域中有什么名称和它们是定义在何处。

全局操作符经常用于测试模块`tests`中，这时会将所有内容引入作用域；我们将在第十一章"How to Write Tests"部分中讲解。全局操作符有时也用于preclude模式；查看"标准库文档"(https://doc.rust-lang.org/std/prelude/index.html#other-preludes)了解更多细节。

























