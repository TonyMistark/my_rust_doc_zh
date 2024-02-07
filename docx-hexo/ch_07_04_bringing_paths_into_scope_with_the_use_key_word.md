---
title: Ch 07.04:全局操作符(The Glob Operator)
date: 2023-08-18 09:25
tags: Rust
layout: Rust
---
#### 全局操作符(The Glob Operator)

如果想将一个路径下所有公有项引入作用域，可以使用指定路径后跟`*`，glob操作符:

```rust
use std::collections::*;
```

这个`use`语句会将`std::collections`中定义的所有的公有项引入到当前作用域。使用全局操作符时一定要多加小心！全局操作符会使得我们难以推导作用域中有什么名称和它们是定义在何处。

全局操作符经常用于测试模块`tests`中，这时会将所有内容引入作用域；我们将在第十一章"How to Write Tests"部分中讲解。全局操作符有时也用于preclude模式；查看"标准库文档"(https://doc.rust-lang.org/std/prelude/index.html#other-preludes)了解更多细节。

























