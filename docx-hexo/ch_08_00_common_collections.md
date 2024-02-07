---
title: Ch 08.00:常见集合
date: 2023-05-18 10:18
tags: Rust
layout: Rust
---
## 常见集合

Rust标准库中包含一些非常有用的数据结构叫做结合(collections).大多数其他的数据类型代表一个特殊的值，但是集合可以包含多个值。不像内建的数组和元组类型，这些集合数据指向堆，意味着在编译时不需要知道数据量，并且可以随着程序运行而增加或减少。每一种集合有不同的性能和消耗，并且选择一个合适的集合，对你当前的情景是需要你慢慢学习的技巧。在本章，我们将会讨论三个集合，它们在Rust编程中很常用：

* vector 允许你一个挨着一个地存储一系列数量的可变的值
* string 是一个字符类型的集合。我们之前已经提到过`String`类型，但是在本章，我们将更加深入地讨论它。
* hash map 允许你用一个特定的key关联一个值(value)，它是成为映射的更通用数据结构的一种实现。

要学习标准库提供的其他类型的集合，请查看文档[std::collections](https://doc.rust-lang.org/std/collections/index.html)

我们将要讨论如何创建和更新vectors, strings，和hash map，以及他们每个类型的独特之处。
