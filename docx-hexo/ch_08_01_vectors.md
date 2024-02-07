---
title: Ch 08.01:丢弃vector 时也会丢弃其所有元素(Dropping a Vector Drops Its Elements)
date: 2023-05-18 10:18
tags: Rust
layout: Rust
---
#### 丢弃vector 时也会丢弃其所有元素(Dropping a Vector Drops Its Elements)

像其他数据结构一样，当一个vector离开了它的作用域，它就会被释放，如Listing 8-10所示：

```rust
fn main() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
```

Listing 8-10: Showing where the vector and its elements are droped

当vector被删除时，它的所有内容也会被删除，这意味着它所持有的整数将被清除。借用检查器确保仅在vector本身有效时才使用对向量内容的任何引用。



接下来，我们将学习下一个集合类型：`String`