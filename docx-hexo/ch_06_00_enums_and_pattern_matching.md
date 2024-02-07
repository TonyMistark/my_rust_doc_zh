---
title: Ch 06.00:Enums and Pattern Matching
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
#### Enums and Pattern Matching

本章中，我们讲述枚举(enumerations)，也称作enums。枚举允许你通过列举可能的成员(variants)来定义一个类型。首先，我们会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。接下来，我们会探索一个特别有用的枚举，叫做`Option`，它代表一个值，要么是某个值要么是什么都不是。然后会讲到`match`表达式中使用模式匹配，针对不同的枚举值编写相应要执行的代码。然后会介绍`if let`，另一个简洁方便处理代码中枚举的结构。

枚举是一个很多语言都有的功能，不过不同语言中其功能各不相同。Rust的枚举与F#，OCaml和Haskell这样的函数式编程语言中的代数数据类型(olgebraic data types)最为相似。

