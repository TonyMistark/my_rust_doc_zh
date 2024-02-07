---
title: Ch 07.02:在模块中对相关代码进行分组(Grouping Related Code in Modules)
date: 2023-05-18 10:18
tags: Rust
layout: Rust
---
#### 在模块中对相关代码进行分组(Grouping Related Code in Modules)

模块让我们在crate中组织代码，以提高可读性和易于重用。模块还允许我们控制items的私有性，因为在module中的代码默认是私有的。Private items是内部实现的细节不对外使用。我们可以选择将模块以及其中的items设置为公共的，这样，外部代码就可以使用并依赖于它们。

例如，让我们编写一个提供餐厅服务功能的library crate。我们将会定义函数的签名，但是函数的主题留空，集中于代码的组织，而不是实现一个餐馆。

在餐厅业中，餐馆的某些部分被称为前台(front of house)，其他的部分称为后台(back of house)。前台是顾客所在的空间；这包括主座的顾客的位置，服务员接单和付款，以及调酒师调酒的地方。后台是主厨和厨师们在厨房，洗碗机清理，经历做行政工作。

这样构建我们的crate，我们可以将这些功能组织成嵌套的模块。创建一个新的库(library)命名为`restaurant`，通过命令`cargo new restaurant --lib`。输入如下Listing 7-1代码到src/lib.rs来定义一些模块和函数签名。这里是前台部分：

src/lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

Listing 7-1: A `front_of_house`module containing other modules that then contain functions

我们用`mod`关键字定义一个模块(在上面的例子中定义了`front_of_house`模块)。并用花括号包围模块的主体。在模块内，我们还可以定义其他模块，就像本例子中的`hosting`和`serving`模块。模块还可以hold一些定义的其他items，例如structs, enums, constants, traits, 以及Listing 7-1中的函数。

通过使用模块，我们可以将相关的定义分组到一起，并指出他们为什么相关。程序设计者可以通过使用这段代码，更加容易找到他们想要的定义，因为他们可以基于分组来对代码进行导航，而不需要阅读所有的定义。程序设计者向这段代码中添加一个新功能时，他们也会知道代码应该放置在何处，可以保持程序的组织性。

在前面我们提到了，`src/main.rs`和`src/lib.rs`叫做crate根。之所以这么叫它们是因为这两个文件中的任何一个内容都在crate模块结构的根组成了一个名为crate的模块，该结构被称为模块树(module tree)。

Listing 7-2 展示了示例Listing 7-1中的模块树(modules tree)的结构。

```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Listing 7-2: The module tree for the code in Listing 7-1

这个模块树展示了一些模块内嵌到另一个模块；例如，`hosting`内嵌在`front_of_house`之中。这个树结构还展示了一些模块是互为兄弟(siblings)的，这意味着它们定义在同一模块中(`hosting`和`serving`被一起定义在`front_of_house`中)。继续沿用家庭关系的比喻，如果一个模块A被包含在模块B中，我们将模块A称为模块B的子模块，B模块为A的父模块。注意，整个模块树都植根于隐式模块名为`crate`之下。

模块树可能会让你想起计算机上的文件系统的目录树；这是一个非常恰当的比较！就像文件系统中的目录一样，你用模块来组织你的代码。就像文件目录中的文件，我们需要一个方式来找到我们的模块(modules)。