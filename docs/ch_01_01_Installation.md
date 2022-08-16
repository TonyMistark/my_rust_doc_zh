#### Installation

#### 安装

##### Linux 或者macOS 执行命令：

```powershell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

这个命令会下载安装脚本工具，并且会安装最新稳定版本的Rust，你有可能需要输入你的密码。如果安装成功就会显示：

```powershell
Rust is installed now. Great!
```

你需要一个链接器(linker)，就是用于将Rust程序编译的输出写入到一个文件。如果你的这个链接器有错误。你需要安装一个C语言编译器。因为Rust很多包都依赖了C语言代码。

##### macOS执行命令：

```powershell
$ xcode-select --install
```

Linux用户通常安装GCC或者Clang，这取决于是哪个发布版本。例如，如果是Ubuntu，你就需要安装build-essential包（搜ubuntu install build-essential基本一搜一大堆。）

##### Windows用户安装

去[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)网址下载，并按照提示安装，过程中会有一些解释信息的提示。

##### 更新和卸载

如果你通过rustup安装的，更新最新版本是最简单的，执行命令：

```powershell
$ rustup update
```

卸载Rust 也是通过rustup, 执行命令：

```powershell
$ rustup self uninstall
```

##### 故障排除

检查Rust安装是否正常，执行命令：

```powershell
$ rustc --version
```

正常情况下会输出版本号，提交哈希串(commit hash)，提交日期等，格式：

```powershell
> rustc x.y.z (abcabcabc yyyy-mm-dd)
```

如果看到以上信息，说明安装正常，如果没有这些信息并且是Windows用户检查你%path%的系统变量。如果%path%是正常的，但是Rust还是不能正常运行，有几个网站也许可以帮到你。

最简单的[#beginners channel on the official Rust Discord](https://discord.com/invite/rust-lang).这里有能帮你的其他Rustaceans(a silly nickname we call ourselves(一个很二的自称绰号))。更多更棒的资源包括[the Users forum](https://users.rust-lang.org/) 和[Stack Overflow](https://stackoverflow.com/questions/tagged/rust).

#### 本地文档

安装过程包括了将对应Rust版本的文档复制到本地。所以你可以阅读官方文档，也可以通过命令：

```powershell
$ rustup doc
```

会在浏览器里打开本地文档。

标准库里面的任何一个类型或者函数你不确定的时候，或者你不知道怎么用。使用应用程序编程接口(API)文档来找出答案。