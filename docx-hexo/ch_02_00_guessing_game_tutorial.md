---
title: Ch 02.00:猜测正确后退出
date: 2022-12-24 13:47
tags: Rust
layout: Rust
---
##### 猜测正确后退出

让我们增加一个break语句，在用户才对时退出游戏：

src/main.rs

```rust
       // --snip--
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

通过在You win!之后增加一行break;，用户猜对了神秘数字之后会退出循环。退出循环也意味着退出程序，因为循环是main最后一部分。

**处理无效输入**

 为了进一步使得游戏表现更好，而不是当用户输入无效数字就崩溃，需要忽略无效输入，让用户继续猜。可以通过修改guess将String转换为u32那部分代码来实现，如Listing 2-5:

src/main.rs

```rust

        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // --snip--
```

Listing 2-5 Ignoring a non-number guess and asking for another guess instead of crashing the program

我们将expect调用换成match语句，以从遇到错误就崩溃变为处理错误。需知parse返回一个Result类型，而Result是一个用于Ok或Err成员的枚举。这里使用的match表达式，和之前处理cmp方法返回Ordering时用的一样。

如果parse不能将字符串转换一个数字，它会返回一个包含更多错误信息的Err。Err值不能匹配第一个match分支的Ok(num)模式，但是会匹配第二个分支Err(_)模式：_是一个通配符值，本例中用来匹配所有的Err值，不管其中有任何信息。所以程序会执行第二个分支的动作，continue 意味着进入loop的下一次循环，请求另一个猜测。这样程序就有效地忽略了parse可能遇到的所有错误！

现在所有的事情都是我们期待的了，让我们试试：

```powershell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

牛逼！再有一个小的修改，就能完成猜数字游戏了：还记得程序依然会打印出秘密数字吗？那个是为了便于测试，但是它破坏游戏性。让我们删掉这个打印语句吧。Listing 2-6是最终版本的代码：

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Listing 2-6 Complete guessing game code

**总结**

至此，你成功地构建了猜数字游戏。恭喜你！

这个项目是一个通过动手实践，向你介绍了很多Rust新概念：let,match,函数，使用外部crate等等。在接下来的章节中，你将会学习这些概念的细节。第三章介绍大部分程序语言都有的概念，比如变量，数据类型和函数，以及如何在Rust中使用他们。第四章探索所有权(owership)，一个特性，使得Rust与其他语言不同的特性。第五章讨论结构体和方法的语法，第六章介绍枚举如何工作。

