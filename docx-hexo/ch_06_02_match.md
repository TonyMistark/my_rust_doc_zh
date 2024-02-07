---
title: Ch 06.02:通用模式和_占位符(Catch-all Patterns and the _ Placeholder)
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
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