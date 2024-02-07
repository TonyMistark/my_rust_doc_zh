---
title: Ch 04.01:返回值与作用域(Return Values and Scope)
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
#### 返回值与作用域(Return Values and Scope)

返回值也可以转移所有权。Listing 4-4展示了一个示例，与Listing4-3一样带有类似的注释。

Filename: src/main.rs

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Listing 4-4: Transferring ownership of return values

变量的所有权总是遵循相同的模式：将赋值给另一个变量时移动它。当持有堆中数据的变量离开作用域时，其值将通过drop被清理，除非数据被移动为另一个变量所有。

虽然这样是可以的，但是在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都穿进去再返回来就有点烦人了，除此之外，我们可能想返回函数中产生的一些数据。

我们可以使用元组来返回多个值，如Listing 4-5。

Filename：src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

Listing 4-5: Returning ownership of parameters

但是，对于一个应该是普通的概念来说，这太多的仪式和大量的工作。幸运的是，Rust有一个特性，可以在不用转移所有权的情况下使用值，它叫做引用(references)。

接下来将讲"引用和借用(References and Borrowing)"

























