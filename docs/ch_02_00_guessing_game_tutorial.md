#### Programming a Guessing Game

**编写一个猜数字游戏**

让我们一起通过完成一个项目，来快速上手Rust。这一章将会给你介绍一点Rust的通用概念，通过在真实的程序里面使用他们。你将会学到关于 let， match，方法(methods)， 关联函数(associate functions), 使用外部crate等等。在接下来的章节中，我们将探索他们的详细细节。在本章，会练习到基础内容。

我们将实现一个经典的新手编程项目：猜猜看。他是如何工作的呢？这个程序会随机生成一个1-100的整数。它会提示玩家输入一个所猜测的数字。当一个猜想被输入，程序会指出所猜数字是太大或者太小。如果猜测正确，这个游戏会打印一个祝贺的信息，并且退出。

准备一个新项目

回到之前创建的projects目录，创建一个新项目。用Cargo：

```powershell
$ cargo new guessing_game
$ guessing_game
```

第一条命令，cargo new，获取guessing_name 作为第一个参数，设置项目名称为guessing_name。第二条命令进入新创建的项目目录。

让我们看看生成的Cargo.toml文件：

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

如你在第一章所见过的，cargo new 为你生成一个"Hello,world!"程序。查看src/main.rs文件：

```rust
fn main() {
    println!("Hello, world!");
}
```

现在让我们用一个命令编译这个"Hello,world!"项目并且运行它，cargo run:

```powershell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Hello, world!
```

run 命令当你需要快速迭代一个项目，像我们将要做的猜数字游戏项目，在进行下一次迭代之前快速测试每一次迭代。

重新打开src/main.rs文件。你将要在这个文件里编写所有的代码。

##### 进行一个猜测

第一部分关于猜数字游戏是要求用户输入，处理这个输入，并检查这个输入是否是你期望的格式。首先，我们允许玩家输入一个猜想。输入代码块Listing 2-1 到src/main.rs

filename:src/main.rs:

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

```

Listing 2-1:Code that gets a guess from the user and prints it

这段代码包含了很多信息，所以让我们来逐行过一遍。包含用户输入和打印结果作为输出，我们需要引入io input/output 库到当前作用域。io库是标准库，也被称作std:

```rust
use std::io;
```

默认情况下，Rust将prelude(https://doc.rust-lang.org/std/prelude/index.html)模块中少量的类型引用到每个程序的作用域中。

如果你需要的类型不在prelude中，你必须使用use显式地将其引入作用域。使用std::io库提供给你的很多有用的特性，包括可以接收用户的输入。

如你在第一章看到的，main函数是项目的入口：

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

fn 语法声明了一个新的函数，小括号(),表明没有参数，并且以花括号{开始函数体。

如你在第一章所学习到的，println!是一个在屏幕打印字符的宏(macro)：

```rust
    println!("Guess the number!");
    println!("Please input your guess.");
```

这两行代码仅仅打印提示用户应该输入什么。

##### 使用变量存储值

接下来，我们创建一个变量存储用户的输入，比如：

```rust
let mut guess = String::new();
```

现在程序变得有意思起来了！在这短短的一行代码里发生了很多事。我们用let声明创建了一个变量。给你另外一个例子：

```rust
let apples = 5;
```

这行命令创建了一个变量名为apples的变量并且绑定了一个值5给它。在Rust里，变量默认是不可变的(immutable)。我们将在第三章讨论这个概念的详细内容[Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability).使得变量可变，我们在变量名前添加mut：

```rust
let apples = 5 // immutable
let mut bananas = 5; // mutable
```

注意：// 语法是开始注释(comments)知道该行结束。Rust将会忽略注释(comments)里的任何东西。我们将在第三章详细讨论注释(comments)。

言归正传，我们回到猜数字游戏程序，你现在知道`let mut guess` 将会引入一个叫做guess的可变变量。等号(=)告诉Rust现在要绑定一个什么到这个变量上。等号右边是guess所绑定的值，它是调用`String::new`的结果，`String::new`的返回值为一个String实例。String(https://doc.rust-lang.org/std/string/struct.String.html)是标准库提供的可增长string类型，UTF-8编码的文本。

`::new`里面的`::`语法表明new是Sting类型的关联函数(associated fuction)。关联函数(associated fuction)是在Sring类型内实现的一个函数。这个new函数创建了一个新的，空的字符串实例。你将会在很多类型里面发现new函数。因为它是一个通用的函数名字，就是创建一个新的变量为某些类型。

总的来说，`let mut guess = Sring::new(); `行就是创建一个可变的变量，用来绑定当前这个新创建的，空的字符串(string)实例。



##### 接收用户输入

回忆一下，我们在第一行用use std::io;从标准库已经引入了输入/输出的实用的功能。我们现在调用io苦衷的stdin:

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

如果我们在一开始没有用`use std::io`导入io标准库，我们仍然可以通过`std::io::stdin`来调用这个函数。stdin函数返回了一个`std::io::Stdin`(https://doc.rust-lang.org/std/io/struct.Stdin.html)的实例(instance)，这个类型表示处理了你终端的标准输入。

下一行，`.read_lin(&mut guess)`在标准库的处理输入的方法read_line(https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)，我们传入了参数&mut guess 给read_line, 告诉它用户输入了一个字符串存在guess变量里。.read_line的全部工作就是无论用户输入什么内容，都将它追加(**不会覆盖其原有内容**)到一个字符串中，因此它需要字符串作为参数。这个字符串参数应该是可变的，以便read_line去修改传入的这个字符串内容。

&表明这个参数是一个引用(reference)，它就是给你一个方法让你代码可变的部分可以访问这一片数据，并且不用把数据复制到内存里面多次。引用是一个复杂的特性，并且也是Rust主要的优势之一，是怎样安全且容易使用引用。你不需要知道太多细节就可以完成这个程序。现在，你所需要知道的就是它像变量一样，默认是不可变的。(第四章将会解释的更加彻底。)

##### 使用Result类型来处理潜在的错误

我们还解析在这行代码，虽然我们已经讲道理文本中的第三行但它仍然是逻辑行（虽然换了行但仍是一条单独的语句）下一段代码是这个方法：

```rust
.expect("Failed to read line");
```

我们也可以写成：

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

然而，过长的代码行难以阅读，所以最好拆开来写。通常来说，只是用.method_name()语法调用方法时引入换行和空格将长的代码行拆开是明智的。现在来看看这行代码做了什么。

之前提到read_line将用户输入通过参数最佳到字符串中，不过它也返回了一个值--在这个例子中是`io::Result`(https://doc.rust-lang.org/std/io/type.Result.html)。Rust标准库中有很多叫做Result类型：一个泛型Result以及在子模块中的特殊版本，比如io::Result。Result类型是枚举(enumerations),通常也写作enums。枚举类型持有固定集合的值，这些值被称为枚举的成员(variants)。枚举通常和match一起使用，这是一个便于根据条件执行时枚举的不同成员值来执行不同代码块的条件语句。

第六章将介绍枚举的更多细节。这里的Result类型将用来编码错误处理的信息。

Result的成员是ok和Err，ok成员表示操作成功，内部包含成功时产生的值。Err成员则意味着操作失败，并包含失败的时为什么失败的信息。

这些Result类型的作用是编码错误处理信息。Result类型的值，想起他类型一样，拥有定义于其上的方法。io::Result的实例拥有expect方法。如果io::Result实例的值是Err，expect会超值程序崩溃，并显示当做参数传递给expect的信息。如果read_line方法返回Err，则可能来源于低层操作系统错误的结果。如果io::Result实例的值是ok，expect会获取ok中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数。

如果不调用expect，程序也能编译，但是会发出警告：

```powershell
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust警告我们没有使用read_line的返回值Result，说明有一个可能的错误没有处理。

消除警告的正确做法就是去编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用expect。第九章会学习如何从错误值恢复。

**使用println!占位符打印值**

除了位于结尾的右花括号，目前为止就只有这一行代码值得讨论一下，就是这一行：

```rust
println!("You guessed: {}", guess);
```

这行代码现在打印了存储用户输入的字符串。第一个参数是格式化字符串，里面的{}是预留给特定位置的占位符：把{}想象成小蟹钳，可以夹住合适的值。使用{}也可以打印多个值：第一对{}使用格式化字符串之后的第一个值，第二对{}则使用第二个值，以此类推。调用一次println!打印多个值看起来像这样：

```rust
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y)
```

这行代码打印结果是: x = 5 and y = 10。

测试第一部分代码

让我们来测试一下猜数字游戏的第一部分。使用cargo run运行：

```powershell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

到此为止，游戏的第一部分已经完成：我们获取从键盘输入的结果，并打印出来。

##### 生成一个秘密数字

接下来，需要生成一个秘密数字，好让用户来猜。秘密数字应该每次都不同，这样重复玩才不会乏味；范围在1-100之间，这样也不会太难。Rust标准库没有生成随机数字的功能。但是，Rust团队还是提供了一个包，包含了这个功能：rand crate(https://crates.io/crates/rand)

###### 使用crate来增加更多功能

记住，crate是一个Rust代码包。我们正在构建的项目是一个二进制crate，它生成一个可执行文件。rand crate是一个库crate,库crate可以包含任意能被其他程序使用的代码，但是不能执行。



Carg对外部crate的运用正是其亮点所在。在我们使用rand编写代码之前，需要修改Cargo.toml文件，引入一个rand依赖。现在打开这个文件并将下面这一行添加到[dependencies]判断标题之下。在当前版本下，请确保按照我们这里的方式指定rand，否则本教程中的示例代码可能无法正常运行。

Cargo.toml

```toml
rand = "0.8.3"
```

在Cargo.toml文件中，标题以及之后的内容都属于一个片段，直到下一个片段的标题才会是下一片段的开始。在[dependencies]中你告诉Cargo项目依赖的外部crate以及其要求的版本。在本例中，我们指定rand crate的版本为0.8.0。Cargo能理解语义控制版本Semantic Versioning(有时候叫SemVer)，这是一个写版本数字的标准。这个数字0.8.3实际上是^0.8.3的简写，意思是0.8.3版本以上0.9.0版本以下。Cargo认为这些发布版本的接口都是兼容0.8.3这个版本的，并且这样的版本确保了我们可以获取能使本章编译的最新补丁版本。任何大于0.9.0的版本不能保证和接下来的示例使用了相同的API。



现在，不修改任何代码，让我们构建这个项目，见Listing2-2

```powershell
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

Listing 2-2:The output from running cargo build after adding the rand crate as a dependency

你也许会看到不同的版本号(但是他们都是兼容的代码，感恩SemVer),同时显示顺序也可能因为操作系统不同而有所不同。

当我们引用了一个外部依赖，Cargo会拉取注册的最新版本的所有的依赖，它就是Crates.io(https://crates.io/)的一个数据拷贝。Crates.io是人们在Rust生态系统里上传的他们的开源Rust项目供他人使用。

在更新完registry之后，Cargo检查[dependencies]片段并下载任何没有被下载下来的crates列表。在本例中，所以我们只下载了rand作为依赖，Cargo同时也抓取rand的依赖的其他依赖。在下载完crates之后，Rust编译了他们，并用这些依赖编译了本项目。

如果不修改任何东西就立刻再次运行cargo biuld，你将不会得到任何输出。Cargo知道它以及下载并且运行了依赖，并且你你没有改过Cargo.toml文件里任何东西。Cargo也知道你没有修改任何代码，所以再也它不会重新编译。因为没有什么事情要做，他就退出了。

如果你打开src/main.rs文件，做一个不重要的修改，并保持它，然后再次build，你仍然只看到两行输出：

```powershell
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

这两行显示Cargo只是针对src/main.rs文件的微小改动而更新构建。依赖没有变化，所以Cargo知道它可以复用已经为此下载并编译的代码。它只是重新构建了项目的部分代码。

**Cargo.lock文件确保构建是可重现的**

Cargo有一个机制，就是确保你能在任何时候rebuild代码，都会产生相同的结果：Cargo只会使用你指定的依赖版本，除了你又手动指定了别的。例如，如果下周rand crate的0.8.4版本出来了，它修复了一个重要的bug,同时也含有一个会破坏代码运行的bug。为了处理这个问题，Rust在你第一次运行cargo build时建立了Cargo.lock文件，我们现在可以在guessing_game目录里找到它。

当你第一次build一个项目的时候，Cargo确定了所有依赖的适合标准的版本并写入了Cargo.lock文件。当你将来build你的项目，Cargo会查看Cargo.lock文件是否存在，并使用指定版本而不是重新确定所有的版本。这让你自然有一个可重现的build。换句话说，你的项目将保持在0.8.3，知道你希望更新，感恩Cargo.lock文件。

**更新Crate到最新版本**

当你确实需要升级crate时，Cargo提供了命令update,它会忽略Cargo.lock文件并且计算所有适合你的在Cargo.toml里的清单的版本。Cargo会将这些版本写入Cargo.lock文件。否则，默认情况下，Cargo只会查看在0.8.3到0.9.0版本之间的版本。如果rand crate已经发布了两个新版本0.8.4和0.9.0，运行cargo update，你会看到：

```powershell
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Cago忽略了0.9.0。这时，你会在你的Cargo.lock文件中注意到一个改变，rand crate的版本现在使用的是0.8.4。要使用0.9.0或者0.9.x系列，你需要更新Cargo.toml文件，用下面的替代:

```toml
[dependencies]
rand = "0.9.0"
```

下一次你运行cargo build，Cargo将会更新可以用crates的注册并重新评估你的rand 要求，根据新版本的指定。

我们将会在第十四章讲更多关于Cargo和它的生态系统，但是现在，这些已经是你全部需要知道的了。Cargo 使得复用库文件非常容易，因此Rustacean能够编写出由很多包组装而成的更轻便的项目。

##### 生成一个随机数

让我们开始使用rand生成一个要用来猜的随机数吧。下一步是更新src/main.rs，如Listing 2-3:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

Listing 2-3:Adding code to generate a random number

首先，我们添加的行使用rand::Rng。Rng是一个trait定义了生成随机数的方法的实现，想使用这些方法，这个trait必须在作用域中。第十章会详细介绍trait。

接下来，我们中间还增加了两行。在第一行里，我们调用了rand::thread函数提供实际使用的随机数生成器：它位于当前执行县城的本地环境，并从操作系统获取seed。接着调用随机生成器的gen_range方法。这个方法由use rang::Rng语句引入到作用域Rng trait定义。gen_range方法获取一个范围表达式(range expression)作为参数，并生成一个在此范围之间的随机数。这里使用的这类范围表达式的在start..end之间取值且包含下限不包含上限，所以需要制定1和101来请求一个1和100之间的数，另外也可以使用范围1..-100，这两者是等价的。

`注意：你不可能凭空就知道应该use那个trait以及该从crate中调用那个方法，因此每个crate都有使用说明文档。Cargo有一个很棒的功能是：运行cargo doc --open命令来构建所有本地依赖提供的文档，并在浏览器中打开。例如假设你对rand crate 中的其他功能感兴趣，你可以运行cargo doc --open 并点击左侧导航栏中的rand.`

新增的第二行打印了秘密数字。这是很有用的，当我们测试开发程序的时候，但是在最终版本，我们要删掉它。如果程序一开始就打印答案，那这就不是游戏了。

尝试运行程序几次：

```powershell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

你应该会得到不同的随机数，并且它们也在1-100之间。干得漂亮！

**比较猜测的数字和秘密数字**

现在，我们有输入的数字和随机数字，我们可以比较它们。步骤在Listing 2-4。注意，这段代码还不能编译，我们将对此进行解释。

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Listing 2-4 Handling the possible return values of comparing two numbers

首先，我们增加了另外一个use声明，从标准库引入一个类型叫std::cmp::Ordering 到作用域。这个Ordering类型是另外一个枚举并有枚举成员Less,Greater,Equal。当你比较两个值的时候可能会有的这三个比较结果。

然后，我们在底部新增五行，使用Ordering 类型。cmp方法用来比较两个值，并且可以在任何可以比较的值上调用。它获取到一个引用，无论你想比较什么：这里比较guess和secret_number。然后它返回一个我们用use声明引入到作用域的Ordering类型的枚举的成员。。我们用一个match表达式决定下一步该做什么，基于调用的cmp通过比较guess和secret_number所返回的Ordering的枚举成员的值。

一个match表达式是由分支(narms)构成。一个分支(arm)包含一个模式(pattern)和表达式开头的值与分支模式相匹配时应该执行的代码。Rust获取提供给match的值并挨个儿检查每个分支的模式。match结构和模式是Rust中强大的功能，它体现了代码可能遇到的多种情形，并帮你确保没有遗漏处理。这些功能将分别在第六章和第十八章详细介绍。

让我们看看使用match表达式的例子。假设用户猜了50，这时随机生成的秘密数字是38.比较50与38时，因为50比38大，cmp方法返回Ordering::Greater。Ordering::Greater是match表达式得到的值。它检查第一个分支点表达式，Ordering::Less与Ordering::Greater并不匹配，所以它忽略了这个分支的代码并来到下一个分支。下一个分支的模式是Ordering::Greater与Ordering::Greater匹配正确！这个分支关联的代码被执行，在屏幕上打印出Too big！。match表达式就此终止，因为该场景下没有必要检查到最后一个分支。

然而，Listing 2-4的代码并不能编译，来试试：

```powershell
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |
   = note: expected reference `&String`
              found reference `&{integer}`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```

错误的核心表明这里有不匹配的类型(mismatched types)。Rust有一个静态强类型系统，同时也有类型推断。当我们写出let guess = String::new()时，Rust推断guess应该是String类型，并不需要我们写出类型。另一方面，secret_number是数字类型。几个数字类型拥有1到100之间的值：32位数字i32；32位无符号数字u32；64位数字164等等。Rust默认使用i32，所以secret_number是i32类型，除非增加类型信息，或任何能让Rust推断出不同数值类型的信息。这里错误的原因在于Rust不会比较字符串类型和数字类型。

所以，我们必须把从输入中读到的String转换成真正的数字类型，才好与秘密数字进行比较。这可以通过在main函数中增加如下代码来实现：

src/main.rs

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

```

这行新代码是

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

这里创建了一个叫做guess的变量。不过等等，不是已经有了一个叫guess的变量了吗？确实如此，不过Rust允许用一个新变量来覆盖(shadow) 之前guess的值。这个功能常用在需要转换类型之类的情况。它允许我们复用guess变量名，而不是被迫创建两个不同变量，像guess_str和guess之类。（第三章会介绍shadowing的更多详情。）

我们将这个新变量绑定到guess.trim().parse()表达式上。表达式中的guess指的是包含输入的字符串类型guess变量。String实例的trip方法会除去字符串开头和结尾的空白字符，我们必须执行此方法才能将字符串与u32比较，因为u32只能包含数值类型数据。用户必须输入enter键才能让read_line返回并输入他们的猜想，这将会在字符串中增加一个换行(newling)符。例如输入5并按下enter(在Windows上，按下enter回到一个回车符和一个换行符，\r\n)，guess看起来像这样：5\n或者5\r\n。\n代表换行，回车键；\r代表回车，回车键。trim方法会消除\n或\r\n，只留下5。

字符串的parse方法将字符串解析成数字，因为这个方法可以解析多种数字类型，因此需要告诉Rust具体数字类型，这里通过let guess:u32指定。guess后面的冒号(:)告诉Rust我们指定了变量的类型。Rust有一些内建的数字类型；u32是一个无符号32位整型。对于不大的正整数来说，它是不错的默认类型，第三章还会讲到其他数字类型。另外，程序中的u32注解以及与secret_number的比较，意味着Rust会推断出secret_number也是u32类型。现在可以使用相同类型比较两个值了！

parse方法只有在数字逻辑上可以转换为数字的时候才能工作，所以非常容易出错。例如，字符串中包含A👍🏻%，就无法将其转换为一个数字。因此parse方法返回一个Result类型。像之前"使用Result类型来处理潜在的错误"(https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type)讨论的read_line方法哪有，在此按部就班地用expect方法处理即可。如果parse不能从字符串生成一个数字，返回一个Result的Err成员时，expect会使游戏崩溃并打印附带错误信息。如果parse成功地将字符串转换成一个数字，它会返回Result的ok成员，然后expect会返回ok中的数字。

现在让我们运行程序：

```powershell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Nice！即便是在猜测之前加了空格，程序依然能判断出用户猜测了76.多运行几次，输入不同的数字来验证不同的行为：猜测一个正确的数字，猜一个过大的数字和猜测一个过小的数字。

现在游戏已经大体能玩了，只不过用户只能猜一次。增加一个循环来实现它吧！

**使用循环来允许多次猜测**

loop关键字创建了一个无限循环。将增加一个无限循环给用户，让用户改变猜测数字：

src/main.rs:

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

如上所示，我们将用户猜测之后的所有内容移动到了循环中。确保循环中的代码多缩进四个空格，再次运行程序。注意这里有一个新问题，因为程序忠实地运行了我们的要求：永远滴神请求一个猜测，用户好像无法退出啊！

用户总能使用Ctrl-c终止程序。不过还有另外一个方法跳出循环，就是"比较猜测数字与秘密数字"部分提到的parse：如果用户输入的答案不是一个数字，程序会崩溃。我们可以利用这一点来退出。如下所示：

```powershell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

输入quit将会退出程序，同时你会注意到其他任何非数字输入也一样。然而这不好，我们想要当猜测正确的数字时游戏停止。

##### 猜测正确后退出

让我们增加一个break语句，在用户才对时退出游戏：

src/main.rs

```rust
       // --snip--
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

通过在You win!之后增加一行break;，用户猜对了神秘数字之后会退出循环。退出循环也意味着退出程序，因为循环是main最后一部分。

**处理无效输入**

 为了进一步使得游戏表现更好，而不是当用户输入无效数字就崩溃，需要忽略无效输入，让用户继续猜。可以通过修改guess将String转换为u32那部分代码来实现，如Listing 2-5:

src/main.rs

```rust

        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // --snip--
```

Listing 2-5 Ignoring a non-number guess and asking for another guess instead of crashing the program

我们将expect调用换成match语句，以从遇到错误就崩溃变为处理错误。需知parse返回一个Result类型，而Result是一个用于Ok或Err成员的枚举。这里使用的match表达式，和之前处理cmp方法返回Ordering时用的一样。

如果parse不能将字符串转换一个数字，它会返回一个包含更多错误信息的Err。Err值不能匹配第一个match分支的Ok(num)模式，但是会匹配第二个分支Err(_)模式：_是一个通配符值，本例中用来匹配所有的Err值，不管其中有任何信息。所以程序会执行第二个分支的动作，continue 意味着进入loop的下一次循环，请求另一个猜测。这样程序就有效地忽略了parse可能遇到的所有错误！

现在所有的事情都是我们期待的了，让我们试试：

```powershell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

牛逼！再有一个小的修改，就能完成猜数字游戏了：还记得程序依然会打印出秘密数字吗？那个是为了便于测试，但是它破坏游戏性。让我们删掉这个打印语句吧。Listing 2-6是最终版本的代码：

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Listing 2-6 Complete guessing game code

**总结**

至此，你成功地构建了猜数字游戏。恭喜你！

这个项目是一个通过动手实践，向你介绍了很多Rust新概念：let,match,函数，使用外部crate等等。在接下来的章节中，你将会学习这些概念的细节。第三章介绍大部分程序语言都有的概念，比如变量，数据类型和函数，以及如何在Rust中使用他们。第四章探索所有权(owership)，一个特性，使得Rust与其他语言不同的特性。第五章讨论结构体和方法的语法，第六章介绍枚举如何工作。

