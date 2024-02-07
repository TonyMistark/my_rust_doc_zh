---
title: Ch 01.03:养成使用Cargo习惯
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
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