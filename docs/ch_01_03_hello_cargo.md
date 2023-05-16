#### Hello,Cargo!

cargo是Rust的构建系统和包管理器。大多数Rustaceans使用这个工具管理他们的Rust项目，因为Cargo帮你处理了很多事情。比如构建你的代码，下载代码依赖库（libraries），构建这些依赖库。（我们把你代码依赖的库叫做依赖(dependencies)）



简单的Rust项目，像刚刚编写的，没有任何依赖。所以如果我们用Cargo构建"Hello,world"项目，那将只会用到Cargo的构建代码的这一部分的功能。随着你编写了更加复杂的Rust项目，你将会添加依赖，并且如果你使用Cargo开始一个项目，添加依赖将会是非常容易做的事情。



因为，大多数Rust项目使用Cargo，接下来这本书假设你也用Cargo。如果你是按照[Installation](https://doc.rust-lang.org/book/ch01-01-installation.html#installation)里的方法安装的Rust，Cargo也会一起被安装。如果你通过其他方式安装的Rust，检查是否安装了Cargo，使用命令：

```powershell
$ cargo --verison
```

如果你看到一个版本号，那么你就已经安装了Cargo。如果你发现如：command not found这样的错误，那么请去找安装Cargo那一部分的文档，确定如何单独安装Cargo。

##### 用Cargo创建一个项目

让我用Cargo创建一个项目并看看和之前创建的"Hello,world!"项目有什么不同。回到project目录（或者任何你存放代码的地方，这个由你决定）。然后，在任何的操作系统上都可以执行命令：

```powershell
$ crago new hello_cargo
$ cd hell_cargo
```

第一条命令创建了一个新目录叫hello_cargo，我们已经叫我们的项目hello_cargo了并且Cargo创建了项目所需的对应的目录和文件。

进入到hello_cargo目录并且ls目录下的文件。你将会看到Cargo已经生成了两个文件和一个目录：Cargo.toml 和src目录以及里面一个文件main.rs。

而且还初始化了一个Git仓库以及一个.gitignore文件。如果你是在一个已经存在的Git仓库中运行cargo new，就不会生成Git文件；可以通过运行cargo new --vcs=git来覆盖这些行为。

`注意：Git是一个常用的版本控制系统（version control system, VCS），可以通过--vcs参数使cargo new 切换到其他版本控制系统（VCS），或者不使用VCS，运行cargo new --help看更多详细用法`

打开Cargo.toml，里面的内容大概如Listing 1-2.

filename:Cargo.toml

 ```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
 ```

Listing 1-2:Contents of Cargo.toml generated by cargo new

这个文件用的是TOML(Tom's Obvios,Minimal Language)格式, 这是一个Cargo的配置文件格式。

第一行[package]，是一个片段标题，表明接下来的声明是配置一个包(package)。随着我们往这个文件里添加更多的信息，我们将会添加其他的片段(section)。

接下来的三行是设置配置文件信息，Cargo需要编译你的程序：项目名字(name)，项目版本(version)，以及你使用的Rust版本。

最后一行，[dependencies]，就是你的项目依赖列表片段的开始。在Rust语言中，代码引用的包就像是一个木板箱(crate:a slatted wooden case used for transporting goods.)，在这个项目里我们不需要任何的crate。但是在Chapter 2的第一个项目里会用到。



现在我们打开文件src/main.rs：

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo 已经帮你生成了一个"Hello,world!"程序。正如我们刚刚Listing 1-1编写的一样，和之前项目不同之处就是Cargo生成的代码放在了src目录下，并且增加了Cargo.toml配置文件。

Cargo期望你将代码放在src目录下。项目根目录就只放如README, license information, Cargo.toml等一些不要依赖你的代码的文件。使用Cargo能帮助你组织你的项目。使得所有的文件都放对地方（There's a place for everything,and ervrything is in its place）。

#### 构建并运行Cargo项目

让我们来看看构建"Hello,world!"项目和构建Cargo创建的hello_cargo有什么不同。构建hello_cargo使用命令：

```powershell
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

cargo build命令创建了一个可执行文件到target/debug/hello_cargo(or target\debug\hello_cargo on windows)而不是在当前目录。你可以运行这个可执行文件，用命令：

```powershell
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

如果一切顺利的话，"Hello, wold!"就应该会打印到终端屏幕上。首次运行cargo build，也会在根目录生成一个Cargo.lock的文件。这个文件用于跟踪你的项目依赖的确切版本。这个项目没有依赖，所以这个文件内容比较少。**你永远不要手动修改这个文件**，让Cargo去管理里面的内容就行了。

我们刚刚用cargo build 构建了一个项目，并且生成了./target/debug/hello_cargo，但是我们也可以使用cargo run来编译并且运行可执行文件，集所有操作于一条命令：

```powershell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

注意这次没有出现表明Cargo正在编译hello_cargo的输出。Cargo发现代码文件并没有改变，所以就直接运行了二进制文件。如果你已经修改了你的源代码文件，Cargo就会在运行它之前重新构建这个项目，并且你会看到这样的输出：

```powershell
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo也提供了一个命令：cargo check。这个命令会快速检查你的代码，确保可以编译但是不会生成一个可执行文件。

```powershell
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

为什么你不想要一个可执行文件？通常，与cargo build相比cargo check 是特别快的，因为它跳过了生成可执行文件的步骤。如果在你写代码的过程中不断地检查，使用cargo check 能为你的开发进程加速。为此，很多Rustaceans们运行会定期cargo check 他们编写的代码，以确保它是可以编译的。当他们需要使用可执行文件的时候，他们就会运行cargo build。

让我们回顾一下关于Cargo，我们都学了些什么：

- 我们可以使用cargo build 构建一个项目。 
- 我们可以使用cargo run一步就可以构建和运行一个项目。
- 我们可以用cargo check来检查一个项目是否可以编译。
- Cargo把编译结果存放在target/debug目录下，而不是当前目录。

另外使用Cargo的一个优势，不管你在什么操作系统，命令都是一样的。所以从现在开始，本书将不再为Linux和macOS以及Windows提供x相应的命令。

**构建发布版本**

当你的项目最后准备发布，你可以使用cargo build --release 来优化编译。这个命令将会创建一个可执行文件到target/release，而不是target/debug。这个优化会使你的代码运行速度更加快。不过启用这个优化会花费更长的编译时间。这也就是为什么会有两种不同的场景：一个是用于开发，当你想经常性的并快速重新编译，而另一个是为了最后你的程序要交付给你一个用户，就不用重复地重新构建而且运行越快越好。如果你在测试代码的运行时间，请确保运行cargo build --release 命令并且测试的可执行文件在target/release。

##### 养成使用Cargo习惯

对于简单的项目，和rustc相比，Cargo不能提供多大的价值，但是对于复杂的项目，安装了很多crates, 它将会证明它的价值所在。让Cargo来协调构建工作会变得容易更多。

即使hello_cargo这么简单的项目，它现在使用了很多你之后的Rust生涯将会用到的使用工具。其实，要在任何已存在的项目上工作时，可以使用如下的命令通过Git拿到代码，移动到你的projects目录，并构建：

```powershell
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

更多关于Cargo详情，请查看Cargo 文档(https://doc.rust-lang.org/cargo/)

**总结**

对于你的Rust之旅，你已经有一个很棒的开始了！本章你已经学会了如何：

- 使用rustup安装最新稳定版Rust
- 更新到新版的Rust
- 打开本地安装的文档
- 直接通过rustc编写并运行"Hello,world!"程序
- 使用Cargo创建并运行项目

是时候通过构建更实质性的程序来熟悉写Rust代码了。所以在Chapter 2,我们将构建一个猜数字的游戏程序。如果你更想从学习Rust常用的编程概念开始，请阅读Chapter 3，接着回来看Chapter 2.