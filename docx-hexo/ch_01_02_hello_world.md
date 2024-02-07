---
title: Ch 01.02:编译和运行是彼此独立的步骤
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
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

