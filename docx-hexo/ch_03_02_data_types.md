---
title: Ch 03.02:无效的数组元素访问
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
###### 无效的数组元素访问

让我们看看发生了什么，如果你尝试访问数组的一个元素，这个元素在数组的结尾之后呢。运行如下的代码，类似于第二章猜数字游戏，从用户输入那里获取数组索引：

src/main.rs

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
```

这段代码编译成功。如果运行这段代码使用cargo run然后输入0，1，2，3，4，这个程序会打印出在数组内对应索引的值。如果你输入一个超出范围的数字，比如10，你会看到输出如：

```powershell
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

当你使用这个无效的值索引的时候，程序的结果是在运行时间的错误。程序退出并返回错误信息，并且没有运行最后的println!语句。当你试图用一个索引访问一个元素，Rust会检查你指定的这个索引是否会超过数组的长度。如果索引与数组长度相同或者更大，Rust就会死给你看。这个检查在运行时间，特别在这个例子中，因为编译器在编译完之后就不可能知道用户会输入什么。

这是一个Rust的内存安全原则示例的表现。在很多底层代码语言中，当你提供一个错误的索引，这种类型的检查它们是不做的，无效内存就会被访问，会导致你不知道你到底访问到了别的什么奇奇怪怪的值。Rust项目中会立即退出，而不是允许你继续访问，从而保护你面授此类错误的影响。第九章将会讨论Rust的错误处理。

