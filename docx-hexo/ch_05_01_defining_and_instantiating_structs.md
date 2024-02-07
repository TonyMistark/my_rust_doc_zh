---
title: Ch 05.01:结构体数据的所有权(Ownership of Struct Data)
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
#### 结构体数据的所有权(Ownership of Struct Data)

在Listing 5-1中的`User`结构体定义中，我们使用了自身拥有所有权的String类型而不是&str字符串slice类型。这是一个有意为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话数据也是有效的。可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上生命周期(lifetimes)，这是一个第十章会讨论的Rust功能。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不是指定生命周期将是无效的，比如这样：

filename: src/main.rs

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

编译器会控诉它需要生命周期标识符：

```rust
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` due to 2 previous errors=
```

第十章会讲到如何修复这个问题以便在结构体中存储引用，不过现在，我们使用像String这类拥有所有权的类型来替代&str这样的引用以修正这个错误。