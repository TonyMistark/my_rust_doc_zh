#### References and Borrowing

Filename:src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
```

Listing 4-5: Returning ownership of parameters

Listing 4-5中的元组代码有这样一个问题：我们必须将String返回给调用函数，以便在调用calculate_length后仍能使用String，因为String被移动到了calculate_length函数内。相反我们可以提供一个String值的引用(reference)。引用(reference)像一个指针，因为它是一个地址，我们可以由此访问存储于该地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值。下面是如何定义并使用一个(新的)calculate_length函数，它以一个对象的引用作为参数而不是获取值的所有权：

Filename: src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

首先，注意变量什么和函数返回值中的所有元组代码都消失了。

其次，主要我们传入了&s1给calculate_length，同时在函数定义中，我们获取&String而不是String。这些&符号就是引用，它们允许你使用值但不获取所有权。Figure 4-5描述了这个概念。

![Figure 4-5](https://mmbiz.qpic.cn/mmbiz_png/ZsCznicobc9I2jzgBsAvXER3icOwwB1ibEVzyRoib7bWsMK6yicZfCAJliaKMrDyrndmWfAIbZ154mZ2VW6ZUGMFnqbw/0?wx_fmt=png)

Figure 4-5: A diagram of &String s pointing at String s1

注意：与使用&引用相反的操作是解引用(dereferencing)，它使用解引用运算符 *。我们将会在第八章遇到一些解引用运算符，并在第十五章详细讨论解引用。

让我们仔细看看这里的函数调用：

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

&s1这个语法让我们创建了一个引用，引用了s1的值，但是没有占有它。因为并不拥有它，当引用停止使用的时候，它指向的这个值不会被drop。

同样地，函数的签名使用&表明参数s的类型是一个引用。让我们添加一些注释：

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
```

变量s有效的作用域与函数参数的作用域一样，不过当s停止使用时并不丢弃引用指向的数据，因为s并没有所有权。当函数使用引用而不是实际的值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。

我们将创建一个引用这个动作叫做借用(borrowing)。就像真实生活中，如果一个人拥有某东西，你可以从他们那里借。当你用完，你可以归还给他们。你并不拥有它。

所以如果我们尝试对我们借来的东西做一些修改，会发生什么呢？Listing 4-6将会尝试。剧透警告：它不会工作。

Finename: src/main.rs

```rust
fn main() {
  let s = String::from("hello");
  change(&s);
}

fn change(some_string: &String) {
  some_string.push_str(", world!");
}
```

Listing 4-6: Attempting to modify a borrowed vale

错误如下：

```rust
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

就像变量默认是不可变的一样，引用也是。我们不允许修改我们引用的东西。

#### 可变引用(Mutable References)

我们可以修改Listing 4-6的代码，以允许修改一个借用的值。这就是可变以后用(mutable reference):

Filename: src/main.rs

```rust
fn main() {
  let mut s = String::from("hello");
  change(&s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

首先，我们修改s为mut。然后我们用&mut s创建一个可变引用的，函数名叫change，并且更新的函数签名some_string: &mut String 使得能访问一个可变引用。这就非常清楚地表明，change函数将改变它所借用的值。

可变引用(mutable reference)有一个很大的限制：你同时只能有一个对某一特定数据的可变引用。这些尝试创建两个s的可变引用的代码会失败：

Filename: src/main.rs

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);
```

错误如下：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 | 
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

这个错误说明这个代码是无效的，因为我们不能借可变的s两次在同一时间。第一次可变借用是r1，并且必须持续到在println!中使用它，但是在那个变量应用的穿件和使用之间，我们又尝试在r2中创建一个可变引用，该引用借用与r1相同的数据。

为了防止和限制多个可变引用同一个数据在同一时间，不过是以一种受限制的方式允许。新来的Rustacean们经常很难适应这一点，因为大部分语言中变量任何时候都是可变的。这个限制的好处是Rust可以在编译时就避免数据竞争。数据竞争(data race)类似于静态条件，它可由这三个行为造成：

* 两个或更多指针同时访问同一数据。
* 至少有一个指针被用来写入数据。
* 没有同步数据访问的机制。

数据竞争会造成未定义(undefined)的表现，并且当你在运行时间尝试追踪错误的时候，很难去分析和修复错误；Rust通过拒绝编译有数据竞争的代码来防止这个问题。

和往常一样，我们可以通过大括号创建一个新的作用域，允许多个可变引用，示例如下：

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
```

Rust为组合可变和不可变引用强制设定了类似的规则。如下的代码会有一个错误：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}	
```

如下为错误：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

当我们有一个不可变的引用的时候，我们不能有一个可变的引用去引用到同样的值。一个不可变引用的用户不希望自己引用的值突然被修改！然而，多个不可变的引用是被允许的，因为没人有能力在读数据的时候会影响到别人读数据。

注意，一个引用的作用域是从被引入开始直到这个引用最后一次被使用之前结束。比如，下面的代码将会编译，因为最后一个使用不可变引用的println!发正在可变引用被引入之前:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

不可变引用r1和r2的作用域结束于println！之后，这也是最后一次被使用，也是在可变引用r3被创建之前。作用域没有重叠，所以这段代码是允许的。编译器在作用域结束之前判断不再使用的引用的能力被称为非词法作用域生命周期(Non-Lexical Lifetings, 建成NLL)。你可以在"The Edition Guide(https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html)"中阅读更多关于它的信息。

即使借用的错误让人很烦，请记住，这是Rust编译器提前指出一个潜在的bug(在编译时间总比在运行时好)并明确展示给你问题的位置。然后你不必去追寻你的数据为什么不是你想要的那样。

#### 悬垂引用(Dangling References)

在指针语言中，很容易会创建一个悬垂指针：就是引用一个内存地址，但是这个地址可能分配给其他所有者。相比直线，在Rust中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

让我们尝试创建一个悬垂引用，Rust会通过一个编译错误来避免：

Filename:src/main.rs

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

错误如下：

```rust
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error

```

错误信息提及了一个我们还没介绍的功能：生命周期(lifetimes)。第十章会详细介绍生命周期。不过，如果你忽略生命周期部分，错误信息中心包含了为什么这段代码有问题的关键信息：

```
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

让我们仔细看看dangle代码的每一步都发生了什么：

Filename:src/main.rs

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

因为s是创建在dangle内，当dangle代码段完成，s将会被释放。但是我们尝试返回一个引用。这意味着这个引用将会是一个无效的字符串。这样不好，Rust也不会让我们这样做。

这里的解决方案是直接返回一个String:

```rust
fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

#### 引用的规则(The Rules of References)

让我们来重新概况一下之前对引用的讨论：

* 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
* 引用必须总是有效的。

接下来，我们来看看另一种不同类型的引用：slice。



















