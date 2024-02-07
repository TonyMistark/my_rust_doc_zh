---
title: Ch 04.00:Understanding Ownership
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
#### Understanding Ownership

Ownership 是Rust最独特的特性，并且对剩下的部分有着深刻的意义。它能让Rust在不需要垃圾回收(garbage collection)就能保证内存安全，所以能够很好地理解owership是如何工作的是非常重要的。在本章，我们将讲和ownership关联的一些特性：borrowing, slice, 和Rust如何在内存中布局数据。

