---
title: Ch 03.04:![allow(unused)]
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
#![allow(unused)]
fn main() {
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
}
```

注释可以放在代码行结尾：

src/main.rs

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today
}
```

但是你经常看到更多是如下的格式，在代码的上方的单独一行注释：

```rust
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
```

Rust 也有其他类型的注释，文档注释(documentation comments)，详细在第14章的"Publishing a Crate to Crates.io" 部分介绍。