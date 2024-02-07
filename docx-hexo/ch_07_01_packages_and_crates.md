---
title: Ch 07.01:包和Crates
date: 2023-05-18 10:18
tags: Rust
layout: Rust
---
## 包和Crates

模块系统的第一部分我们将介绍包和crate。

crate是Rust编译时最小的代码单位。如果你用`rustc`而不是`cargo`来编译一个文件(第一章我们这么做)，编译器还是会将那个文件认作一个crate。crate可以包含模块，模块可以定义在其他文件，然后和crate一起编译，我们会在接下来的章节中遇到。

crate有两种形式：二进制项和库。二进制crate可以被编译为可执行程序，比如一个命令行程序或者一个服务器。它们必须有一个`main`函数来定义当程序被执行的时候所需要做的事情。到目前为止，我们所创建的crate都是二进制crate。

库(Library crates) 没有`main`函数，而且它们不会编译成可执行文件。相反，它们定义了旨在与多个项目共享的功能。例如，我们在第二章用过的`rand`函数，用来生成随机数。大多数的Rust开发者说的crate指的都是库，这与其他变成语言中的library概念一致。

crate root是一个资源文件，Rust编译器以它为起始点，并组成crate的根模块(我们将在"Defining Modules to Control Scope and Privacy" 部分深度解析)。

包(package)是提供一系列功能的一个或者多个crate。一个包会包含Cargo.toml文件，阐述如何去创建这些crates。Cargo就是一个包含构建你代码的二进制项的包。Cargo也包含这些二进制项所依赖的库。其他项目也能用Cargo库来实现与Cargo命令行程序一样的逻辑。

包可以包含很多二进制(binary) crate，但是最多只能有一个库(library) crate。包无论是一个库(library crate)还是一个二进制库(binary crate)， 至少要包含一个library crate。

让我们来看看创建包的时候会发生什么。首先，我们输入命令`cargo new`：

```shell
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

运行`cargo new`之后，我们使用`ls`查看Cargo创建了什么，在这个项目目录里，Cargo会给我们创建一个Cargo.toml文件，这就是一个包(package)。这里也有一个src目录包含main.rs文件。用编辑器打开Cargo.toml，并且文件内容里面没有提及src/main.rs。因为Cargo遵循的一个约定：src/main.rs就是一个与包同名的二进制crate的crate根。同样地，Cargo知道如果包目录中包含src/lib.rs，则包带有与其同名的库(library) crate。crate根文件将由Cargo传递给`rustc`来构建库(library)或者二进制(binary)。

这里，我们有一个只包含src/main.rs的包，意味着它只含有一个名为`my-project`的二进制crate。如果一个包同时含有src/main.rs和src/lib.rs，则它有两个crate：一个二进制(binary)的和一个库(library)，并且名字都与包相同。通过将文件放在src/bin目录下，一个包可以拥有多个二进制crate：每个src/bin下的文件都会被编译成一个独立的二进制crate。