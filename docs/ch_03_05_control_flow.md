#### 控制流(Control Flow)

能运行一些代码取决于是否某一个条件是真(true)，或者当一个条件为真(true)重复运行一些代码，是大多数编程语言基本的组成部分。Rust代码中最常见的用来控制执行留的结构是if表达式或者循环(loops)。

#### if 表达式

一个if表达式允许你根据条件执行不同的代码分支。你提供一个条件并表示"如果条件满足，运行这段代码；如果条件不允许，不运行这段代码。"

在你的projects目录创建一个新的项目叫作branches，来探索if表达式。在src/main.rs文件输入如下代码：

src/main.rs

```rust
fn main() {
    let number = 3;
    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
  }
}
```

所有的if表达式都是以关键字if开始，然后跟着一个条件。在这个示例中，条件检查是否是一个数字变量并绑定的值小于5。如果条件是true，在这个条件后面的大括号内的代码块会马上执行。代码块与条件关联，if表达式优势也叫做arms，就像第二章"Comparing the Guess to the Secret Number"部分讨论到的match表达式的分值一样。

我们也可以包含一个可选的else表达式，当我们这么做了，在条件为false时给程序一个可替代的代码块来执行。如果你不提供else表达式并且条件是false，程序会跳过if块并移动到下一段代码。

试试运行这段代码；你会看到如下的输出：

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was true

```

让我们尝试修改number的值使条件变为false，看看会发生什么：

```rust
let number = 7;
```

再运行这个程序，输出如下：

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was false
```

`值得注意的是条件语句必须是bool类型`如果条件不是bool，将会报错，比如下面的示例：

src/main.rs

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

这次if条件的值为3，Rust会抛出一个错误：

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

错误标兵Rust期待一个bool但是得到一个integer。不像Ruby和JavaScript这样的语言，Rust不会自动将非Boolean类型的值转换为一个Boolean类型。你必须显式并总是提供一个Boolean给if条件。例如，如果我们想if代码块运行只当一个数字不等于0，我们可以修改这个if表达式如下：

```rust
fn main() {
    let number = 3;
    if number != 0 {
        println!("number was someting other than zero");
    }
}
```

运行这段代码将会打印`number was someting other than zero`。

#### 处理多个else if 条件(Handing Multiple Conditions with else if)

在if表达式中你可以通过结合if和else使用多个条件。例如：

src/main.rs

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

这个程序有四个可能的路径可以选。在运行它之后，你会看到如下的输出：

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number is divisible by 3
```

当这个程序执行的时候，它会按顺序检查每个if表达式并执行第一个条件为真的代码块。注意即使6可以被2整除，也不会输出number is divisible by 2，更不会输出else块中的number is not divisible by 4, 3, or 2。因为Rust周会执行第一个条件为真的代码块，并且一旦它找到一个以后，甚至都不会检查剩下的语句了。

使用太多的else if表达式会使得你的代码变得杂乱，所以如果你有超过一个else if的时候，你也许就应该想想重构你的代码了。第六章会讲一个强大的叫做match的东西。

#### 在let语句中使用if

因为if是一个表达式，我们可以在let语句的右边使用它来分配一个变量的输出，如Listing 3-2。

src/main.rs

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

Listing 3-2: Assigning the result of an if expression to a variable



number 变量将会绑定一个值，基于if表达式的输出。运行这段代码，你会看到如下输出：

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5
```

记住，代码块的计算结果是其中的最后一个表达式，而且数字本身也是一个表达式。在这个例子中，整个if表达式的值取决于代码块的执行。这意味着if的每个分支的可能的返回值都必须是相同类型；在Listing 3-2中，if分支和else 分支的结果都是i32整型。如果他们的类型不匹配，如下这个例子中，则会出现一个错误：

文件名：src/main.rs

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
```

当我们尝试编译这段代码，我们将会得到一个错误。if 和else分支的值类型不兼容，而Rust也准确地支出在冲虚中的何处发现的这个问题：

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

表达式if 块计算出的值是一个整数，表达式else块计算得出一个string，这是不会运行的，因为变量必须只有一个单个类型，Rust在编译时间需要确切地知道number变量的类型是什么。如果number的类型仅在运行时确定，则Rust无法做到这一点；并且编译器必须跟踪每一个变量的多重假设类型，那么它会变得更加复杂，对代码的保证也会减少。

#### 使用循环重复执行(Repetition with Loops)

多次执行同一段代码是很常用的。为了这个任务，Rust提供了集中loops，它将会运行在loop主体内的代码直到结束并立即回到开始再次执行。为了体验loops，让我们创建一个新的项目叫loops。

Rust有三种类型的循环(loops)：loop, while, for。让我们来一个一个试试。

#### 使用loop重复执行代码(Repeating Code with loop)

loop关键字告诉Rust一遍又一遍执行这块代码直到永远，直到你明确地告诉他停止。

如下示例，修改src/main.rs文件到你的loops目录：

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

当我们运行这个程序，会看到again!被不停地打印直到我们手动停止它。大多数终端(terminal)支持快捷键ctrl-c来终止陷入无限循环的循环。来试试：

```shell
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

^C标志表明你使用了ctrl-c。你也许会或者也许不会看到才^C后面打印again!，它取决于当循环收到终端信号的时候代码是不是正好在循环里面。

幸运的是，Rust也提供了通过用代码终端玄幻的方法。你可以使用break关键字在循环内，告诉程序什么时候停止执行循环。回想一下第二章猜数字游戏"Quitting After a Correct Guess"部分的程序，当用户通过才对数字赢了是如何退出程序的。

我们也可以在猜数字游戏中使用continue，循环中的continue关键字告诉程序跳过这个循环迭代中的任何剩余代码，并转到下一个迭代。

如果存在嵌套循环，break和continue应用于此时最内层的循环。你可以选择在一个循环上指定一个循环标签(loop label)，然后将标签与break或continue一起使用，使这些关键字应用于一标记的循环而不是最内存的循环。下面是一个包含两层循环的示例：

```
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

外面的循环有一个label `'counting_up`，它将从0数到2.没有表情的内部循环从10向下数到9.第一个没有指定标签的break将只退出内层循环。break `'counting_up;`语句将退出循环。这个代码的输出如下：

```shell
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

#### 从循环中返回值(Returning Values from Loops)

loop的一个用例是重试可能会失败的操作，比如检查程序是否完成了任务。然而你可能会需要操作的结果传递给其他的代码。如果将返回值加入你用来停止循环的break表达式，它会被停止的循环返回：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

在循环之前，我们声明了一个名为counter的变量并初始化为0。接着声明了一个名为result来存放循环的返回值。在循环的每一次迭代中，我们将counter变量加1，接着检查counter是否等于10。当等于时，使用break关键字返回值counter * 2。循环之后，我们通过分号结束复制给result的语句。最后打印出result的值。也就是20。

#### while条件循环(Conditional Loops with while)

在程序中计算循环的条件也很常见。当条件为真，执行新婚换。当条件为假，调用break停止循环。这个循环类型可以通过组合loop、if、else和break来实现；如果你喜欢的话，现在就可以在程序中试试。

然而，这个模式太常用了，Rust为此内置了一个语言结构，它被称为while循环。Listing3-3使用了while：程序循环三次，每次数字都减一。接着，在循环结束后，打印出另外一个信息并退出。

文件名：src/main.rs

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

Listing 3-3 Using a while loop to run code while a condition holds true

这种结构消除了很多使用loop、if、else和break时所必须的嵌套，这样更加清晰。当条件为真就执行，否则退出循环。

#### 使用for遍历集合(Looping Trough a Collection with for)

你可以使用while结构来循环一个集合的元素，如一个数组，例如Listing 3-4中循环数组a的每个元素：

文件名：src/main.rs

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

Listing 3-4 Looping through each element of a collection using a while loop

这里，代码对数组中的元素进行技术。它从索引0开始，并借着循环知道遇到数组的最后一个索引(这时，index < r不再为真)。运行这段代码会打印出数组中的每个元素：

```rust
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

数组中的5个元素都在终端被打印出来，如你所期待的那样。尽管索引索引在某一时刻会到达5，循环会在尝试从数组中拿第6个值的时之前停止。

然而，这个接近是错误的倾向；如果索引的值或者测试条件是错误的，我们可能会造成程序报错。例如，如果你修改了a数组的定义，只有四个元素，但是你忘了更新条件为while index < 4，那么这个代码就会报错。这也是程序更慢，因为编译器增加了运行时代码对每次循环进行检查，以确定循环的每次迭代中索引是否在数组的边界内。

作为更简洁的替代方案，可以使用for循环来对一个介个的每个元素执行一些代码。for循环代码如Listing 3-5。

文件名：src/main.rs

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
```

Listing 3-5:Looping through each element of a collection user a for loop

当我们运行这段代码，我们会看到如Listing3-4一样的输出。更加重要的是，我们增强了代码安全性，并消除了可能由于超出数组的结尾或遍历长度不够而缺少一些元素导致的bug。

例如，在Listing 3-4代码中，如果你将a数组的定义改为4个元素，但忘记了修改条件为while index < 4，代码将会报错。使用for循环的话，就不需要惦记着改变数组元素是修改其他行的代码了。

for循环的安全性和间接性使得它成为Rust中使用最多的循环结果。即使是在想要循环执行代码特定次数时，如Listing3-3中使用while循环倒计时例子，大部分Rustacean也会使用for循环。这么做的方式是使用Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。

下面是一个使用for循环来倒计时的例子，它还使用了一个我们还未降到的方法，rev，用来翻转range:

文件名：src/main.rs

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

这段代码是不是更好呢？

#### 总结(Summary)

你做到了！这是相当大的一章：你学习了关于variables, scalar and compound data type, functions, comments, if expressions, and loops! 讨论并练习这些概念。尝试构建如下的程序：

* 相互转换摄氏与华氏温度(Convert temperatures between Fahrenheit and Celsius.)
* 生成n阶斐波那契数列(Generate th nth Fibonacci number.)
* 打印圣诞颂歌词，并利用歌词中的重复部分(编写循环)(Print the lyrics to the Christmas carol "The Twelve Days of Christmas," taking advantage of the repetition in the song.)

当你准备好继续的时候，让我们讨论一个其他语言中不常见的概念：所有权(ownership)

