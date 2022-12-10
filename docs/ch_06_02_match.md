## Match 的控制流结构

Rust有一个非常强大的控制流结构，称为`match`，它允许你将值与一系列模式进行比较，然后根据匹配的模式执行代码。模式可以由字面量，变量名，通配符，以及其他的东西。第18章涵盖了所有不同类型的模式及其他们的行为。`match`的力量来源于模式的表现力以及编译器检查，它确保了所有可能的情况都得到处理。

可以把`match`表达式想象成某种硬币分类器：硬币滑入有着不同大小孔洞的轨道，每个硬币都会掉入符合它大小的孔洞。同样地，值也会通过`match`的每一个模式，并且在遇到第一个"符合"的模式时，值会进入相关联的代码块并在执行中被使用。



说到硬币，让我们用它来作为一个使用`match`的例子！我们可以编写一个函数来获取一个位置的硬币，并以一种类似验钞机的方式，确定它是何种硬币并返回它的面值，如Listing 6-4所示。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```

Listing 6-3: An enum and a `match` expression that has the variants of the enum as its patterns

让我们拆开函数`value_in_cents`里的`match`看看。首先，我们列出`match`关键字后跟一个表达式，在这个例子中是`coin`的值。这看起来和`if`表达式的使用很像，但是这里有巨大的不同，`if`表达式需要返回一个Boolean类型的值，但是这里，可以返回任何类型。例子中`coin`的类型是我们第一行定义的`Coin`枚举类型。

接下来是`match`的分支。一个分支有两个部分：一个模式和一些代码。第一个分支的模式是值`Coin::Penny`之后的`=>`的运算符将模式和将要运行的代码分开。这里的代码仅仅是值`1`。每一个分支之间使用逗号分隔。

当`match`表达式执行时，它将结果值按顺序与每一个分支的模式比较。如果模式匹配到了这个值，这个模式相关联的代码将会执行。如果模式并不匹配这个值，将继续执行下一个分支，非常类似一个硬币分类器。根据需要你可以拥有任意多的分支：Listing 6-3，我们的`match`有四个分支。

与每一个分支相关联的代码是一个表达式，而表达式的结果值将作为整个`match`表达式的返回值。

如果分支代码短的话，通常不适用大括号，就像Listing 6-3中的每个分支只返回一个值。如果你想在match分支中运行一个多行的代码，你必须使用大括号，而分支后的逗号是可选的。例如，下面的代码每次使用`Coin::Penny`调用时都会打印"Lucky penny!"，同时仍然返回代码块最后的值`1`：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}

```

#### 绑定值模式(Patterns that Bind to Values)

match分支的另外一个有用的特性，是可以绑定匹配的模式的部分值。这也就是如何从枚举成员中提取值的。

作为一个例子，让我们修改枚举的一个成员来存放数据。1999到2008年间，美国的25美分在50个州硬币的一侧都有不相同的设计。其他的硬币都没有这种区分州的设计，所以这些25美分硬币有特殊的价值。可以将这些信息加入我们的`enum`，通过改变`Quarter`成员来包含一个`State`值，示例如下Listing 6-4。

```rust
#[derive(Debug)] // so we can inspect the state in a minute
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

fn main() {}
```

Listing 6-4 A `Coin`enum in which the `Quarter` variant alse holds a `UsState` value

让我们想象一下，一个朋友正在努力收集所有50个州的硬币。当我们把零钱按照硬币的种类分类，我也可以说出每个硬币所关联州的名字，这样如果我们的朋友没有的话，他可以将它加入收藏。

在这些代码的匹配表达式中，我们添加一个变量叫`state`的成员到模式匹配`Coin::Quarter`成员的分支的模式中，变量`state`将会绑定25美分硬币所对应州的值。接着在那个分支的代码中使用`state`，如下：

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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

如果调用`value_in_cents(Coin::Quarter(UsState::Alaska))`,`coin`将是`Coin::Quarter(UsState::Alaska)`。当将值与每个分支相比较时，没有分支会匹配，直到遇到`Coin::Quarter(state)`。这时，`state`绑定的将会是值`UsState::Alaska`。接着就可以在`println!`表达式中使用这个绑定了，像这样就可以获取`Coin`枚举的`Quarter`成员中内部的州的值。

#### Mathcing with Option\<T\>

我们在之前的部分使用`Option<T>`时，是为了从`Some`中取出其内部的`T`值；我们可以像处理`Coin`枚举那样使用`mathc`处理`Option<T>`。只不过这回比较的不再是硬币，而是`Option<T>`的成员，但`match`表达式的工作方式不变。

比如我们想要编写一个函数，它获取一个`Option<i32>`，如果其中含有一个值，将其加1。如果其中没有值，函数应该返回`None`值，而不尝试执行任何操作。

感谢`match`，让我们编写这个函数变得非常容易，它将看起来像Listing 6-5。

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

Listing 6-5: A function that uses a `match` expression on an `Option<i32>`

让我们仔细检查`plus_one`的第一行操作。当调用`plus_one(five)`时，`plus_one`函数体重的`x`将会是值`Some(5)`。接着将其与每个分支比较。

```rust
						None => None,
```

值`Some(5)`并不匹配模式`None`，所以继续进行下一个分支。

```rust
						Some(i) => Some(i + 1),
```

`Some(5)`与`Some(i)`匹配吗？当然匹配！他们是相同的成员。`i`绑定了`Some`中包含的值，所以`i`的值时`5`。接着匹配分支的代码被执行，所以我们将`i`的值加一并返回一个含有值`6`的新`Some`。

接着让我们思考第二次调用`plus_one`在Listing 6-5中，`x`是`None`。我们进入`match`并与第一个分支相比较。

```rust
						None => None,
```

匹配上了！这里没有加值， 因此程序停止并在`=>`右边返回`None` 。因为第一个分支匹配到了，不会去匹配其他分支了。

将`mathc`与枚举相结合在很多场景中都是有用的。你会在Rust代码中看到很多这样的模式：`match`一个枚举，绑定其中的值到一个变量，接着根据其值执行代码。这在一开始有点复杂，不过一旦习惯了，你会希望所有语言都拥有它！这一直是用户的最爱。

#### 匹配是穷尽的(Matches Are Exhaustive)

`match`还有另一方面需要讨论：这些分支必须覆盖所有的可能性。考虑一下我们的`plus_one`函数如下的版本，有一个bug,是不能编译通过的：

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

我们没有处理`None`的情况，所以这个代码会产生一个bug。幸运地是，这是一个Rust知道如何处理的bug。如果尝试编译这段代码，会得到这样一个错误：

```rust
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```

Rsut知道我们没有覆盖所有可能的情况，甚至知道那些模式被忘记了！Rust中匹配是穷尽的(exhaustive)：我们必须穷举所有的可能性来使得代码有效。特别的在这个`Option<T>`的例子中，Rust防止我们忘记明确的处理`None`的情况，这让我们免于假设拥有一个实际上为空的值，从而使之前提到的价值亿万的错误不可能发生。

#### 通用模式和_占位符(Catch-all Patterns and the _ Placeholder)

使用枚举，我们希望对一些特定的值采取特殊操作，而对其他的值采用默认操作。想象我们正在玩一个游戏，如果你掷出一个骰子的值为3，角色不会移动，而是会得到一顶新奇的帽子。如果你掷出了7，你的角色将会失去新奇的帽子。对于其他的数值，你的角色会在棋盘上移动想的格子。这是一个实现了上述逻辑的`match`。骰子的结果是硬编码而不是一个随机值，其他的逻辑部分使用了没有函数体的函数来表示，实现它们超出了本例的范围：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
```

对于前两个分支，匹配模式是字面值3和7，最后一个分支则涵盖了所有其他可能性的值，模式使我们命名为`other`的一个变量。`other`分支的代码通过将其传递给`move_player`函数来使用这个变量。

即使我们没有列出`u8`所有可能的值，这段代码依然能够编译，因为最后一个模式将匹配所有未被特殊列出的值。这种通配模式满足了`match`必须穷尽的要求。请注意，我们必须将通过通配分支放在最后，因为模式是按顺序匹配的。如果哦我们在通配分支后添加其他分支，Rust将会警告我们，因为此后的分支永远不会被匹配到。

Rust还提供了一个模式，当我们不想使用通配模式的值时，请使用`_`，这是一个特殊模式，可以匹配任意值而不用绑定到该值。这告诉Rust我们不会使用这个值，所以Rust也不会警告我们存在未使用的变量。

让我们改变游戏规则：现在，当你掷出的值不是3或7的时候，你必须再次掷出。这种情况下我们不需要使用这个值，所以我们改动代码使用`_`来替代变量`other`：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
```

这个例子也满足穷举性要求，因为我们在最后一个分支中明确地忽略了其他的值。我们没有忘记任何东西。

最后，我们再次改动游戏规则，如果投掷出3或者7以外的值，你的回合将无事发生。我们可以使用单元值(unit value：就是空的tuple，在"The Tuple Type"部分有提及)作为`_`分支的代码：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
```

在这里，我们明确告诉Rust我们不会使用与前面模式不匹配的值，并且这种情况下我们不想运行任何代码。

更详细的模式匹配相关我们将会在Chapter 18进行讲解。目前，我们继续讨论`if let`语法，这在`match`表达式有点冗长的情况下比较有用。