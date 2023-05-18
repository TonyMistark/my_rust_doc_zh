## if let 简洁控制流

 `if let`的语法让你结合`if`和`let`变成一个简洁的方式来处理值，只匹配一种模式，其他模式忽略的情况。思考Listing 6-6程序，匹配一个`config_max`变量中的 `Option<u8>`值，但是只想执行值是`Some`的代码。

```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
```

Listing 6-6: A `match` that only cares about executing code when the value is `Some`

如果值是`Some`，我们希望打印出`Some`成员中的值，这个值被绑定到模式中的`max`变量里。对于`None`值我们不希望有任何操作。为了满足`match`表达式(穷尽性)的要求，在处理完只有一个成员后面必须添加`_ => ()`，这是让人讨厌的代码模版。

然而，我们可以使用`if let`来编写简短的代码。如下代码与Listing 6-6中的`match`行为一致：

```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```

`if let`语法采用一个模式和一个等号分隔的表达式。它和`match`是相同的运行方式，其中表达式被赋予`match`，而模式是第一个分支。在这个例子中，模式是`Some(max)`，并且`max`绑定在`Some`之中。然后就可以在`if let`代码块中使用`max`，和在对应的`match`分支中使用`max`时相同的。如果值没有匹配到相应的模式，`if let`代码块是不会执行的。

使用`if let`意味着写更好的代码，更少的缩进和更少的模版代码。然而，这样会失去`mathc`强制要求的穷尽性检查。`match`和`if let`的使用选择取决于你在你的特定情况下要做什么，是否要遵循简洁原则同时也会失去穷尽检查特性，两者之间的一个平衡。

换句话说，你可以认为`if let`作为`match`的一个语法糖，当运行只有一个匹配模式的情况并且忽略其他所有的值的情况就可以使用。

当使用`if let`的时候，我们可以包含一个`else`。进入到`else`代码块时的情况和`match`表达式中的`_`情况一样，这样的`match`表达式等同于`if let`和`else`。回忆一下Listing 6-4中`Coin`枚举的定义，其中`Quarter`成员也包含一个`UsState`值。如果想要计数所有不是25美分的硬币的同时也报告25美分硬币所属的州，可以使用这样一个`match`表达式：

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
```

或者可以使用`if let`和`else`表达式：

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```

如果你的程序遇到一个使用`match`表起来过于啰嗦的逻辑，记住`if let`也在你的Rust工具箱中。

### 总结

现在我们学习了如何使用枚举创建自定义类型。我们也展示了标准库的`Option<T>`类型是如何帮助你利用类型系统来避免出错的。当枚举值包含数据时，你可以根据你需要处理多少种情况来选择使用`mathc`或者`if let`来提取或者使用这些值。

你的Rust程序现在能够使用结构体和枚举在你自己的作用域内表现其内容了。在你的API中使用定义类型保证了类型安全：编译器会确保你的函数只会得到它期望的类型的值。

为了提供一个有条理的API给你的用户，它使用起来很简单易懂，值暴露了你的用户需要的东西，现在我们准备开始学习Rust的模块。

