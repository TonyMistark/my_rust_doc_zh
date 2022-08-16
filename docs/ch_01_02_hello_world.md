#### Hello,World

现在你已经安装了Rust，让我们一起写你的第一个Rust程序吧！每当学习一个新的语言都有一个优良传统，那就是打印一条"Hello, World!"的文本到屏幕上。所以现在开始。

提示：这个本书假定读者对命令行有个基本的了解。Rust对编辑器没有特别的要求，所以如果你喜欢用IDE(integrated development environment)，而不是用命令行。那么请随意使用你的IDE。很多IDE都不同程度会对Rust有支持，详情请查看你所用的IDE的文档。近期，Rust团队也开始专注提供强大的IDE支持。并且在这方面取得了快速的进展。

##### 创建一个项目目录

你将创建一个目录来存放你的Rust代码。代码放在什么地方都是没问题的，但是对这本书的练习和项目来说，我们建议你在home目录创建一个projects的目录，并且将所有项目都放在这个目录里面。

打开终端，输入厦门的命令行，创建一个projects目录并创建一个存放"Hello,World!"项目的目录。

对于Linux, macOS 和 PowerShell on Windows, 输入：

```powershell
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello world
```

##### 编写和运行一个Rust程序

接下来，创建一个源文件并命名为main.rs。Rust 文件都是以.rs为后缀名的。如果你用一个超过一个单词的文件名，使用下划线连接。例如 hello_world.rs而不是helloworld.rs

现在打开main.rs文件，并且输入Listing1-1的代码：

```rust
fn main() {
    println!("Hello, world!");
}
```

Listing 1-1: Aprogram that Hello, World!

保存文件，并回到终端窗口。对于Linux或者macOS用户输入如下的命令并运行该文件：

```powershell
$ rustc main.rs
$ ./main
Hello, world!
```

无论你什么系统，"Hello,world!"都会打印到你的终端上。如果你没看到这样的输出，回到[Troubleshooting](https://doc.rust-lang.org/book/ch01-01-installation.html#troubleshooting)"故障排除"片段寻找帮助。

如果"Hello, world!"打印了，恭喜你！你正式编写了一个Rust程序。这将使你成为了一个Rust程序员了，欢迎！欢迎！(大家呱唧呱唧👏🏻)

##### 分析该Rust程序

让我们来回顾一下这个"Hello，world!"程序到底发生了什么。这是第一块拼图：

```rust
fn main() {
}
```

这两行定义了一个Rust函数。main是一个特殊的函数：它在任何可运行的Rust程序中，永远是第一个开始运行的代码。第一行声明了函数名main，没有参数也没有返回值。如果有参数，他们应该放在()之中。

还需要注意的是，函数主体应该被包裹在花括号{}之中。Rust要求所有的函数主体都包括在花括号{}内。将左花括号{和函数法名放在一行并以一个空格隔开是比较好的代码风格。

如果你想在Rust项目中保持一个标准的代码风格，你可以使用叫做rustfmt的自动格式化工具来保证一个统一的代码风格。Rust团队已经在标准的发布版本包括了这个工具。像rustc,所以它应该已经安装到你的电脑里了，查看在线文档了解更多。

main函数内部包含的代码如下:

```rust
println!("Hello, world!");
```

这行代码做了这个小程序的所有的工作：它打印文本到屏幕。这里有四个重要的细节需要注意：

首先，Rust缩进风格是4个空格而不是tab。

第二，println!调用了Rust宏(macro)。如果它调用的是一个函数，它将输入println(没有"!")，我们将在Chapter 19讨论Rust宏(macro)的细节。现在，你只需要知道用!表示你调用了一个宏(macro)，而不是普通函数。而且宏(macro)并不总是遵循与函数相同的规则。

第三，你看到的"Hello,world!"字符串。我们把这个字符串作为参数传递给了println!,并且这个字符串打印到了屏幕上。

第四，我们以分号";"结束了这一行。它代表了这一段表达式的结束以及下一段表达式的开始。大多数Rust代码都是以分号";"结尾。

##### 编译和运行是彼此独立的步骤

你刚刚运行了一个新的程序，所以让我们来检查一下这个过程的每一个步骤。

在运行Rust程序之前，你必须通过rustc命令，用Rust编译器来编译这个它，并传入你源代码文件的名字，比如：

```powershell
$ rustc main.rs
```

如果你有C或者C++的技术背景，你可以视作像gcc 或者clang。在编译成功之后，Rust 会输出一个二进制可执行文件。

对于Linux,macOS和PowerShell on Windows,执行ls的shell命令之后，你就可以看到这个可执行文件。对于Linux，macOS，你可以看到两个文件。对于PowerShell on Windows,你会看到和使用CMD一样的三个文件。

```powershell
$ ls
main main.rs
```

在Windows的CMD，输入命令：

```powershell
$ dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

这显示出以.rs为扩展名的源代码文件和可执行文件(main.exe on Windows, but main on all other platforms),并且，当使用Windows，一个文件包括了debugging信息的.pdb扩展名。到此，你可以开始运行main 或者main.exe文件，如：

```powershell
$ ./main # or .\main.exe on Windows
```

如果main.rs是你的"Hello,world!"程序，这行将打印出"Hello,world!"到你的屏幕。

如果你更加熟悉动态语言，如Ruby,Python, JavaScript, 你也许不习惯将编译和运行分为两个单独的步骤。Rust是一个预编译(ahead-of-time compiled)语言，代表你可以编译一个程序并且给可执行文件其他人，并且他们可以直接运行，不需要安装Rust。如果你给某人一个a.rb, .py, .js文件，他们需要安装一个Ruby,Python 或者JavaScript 实现(运行环境)。不过这些语言中，只需要一句命令就可以编译和运行程序。所有的事都是语言设计上的取舍。

仅仅使用rustc编译简单程序是没有问题的。但是随着你的项目发展，你会想要管理所有的选项，使得更简单地共享你的代码。接下来，我们将介绍给你一个叫做Cargo的工具，它将帮助你编写真实世界的Rust程序。

