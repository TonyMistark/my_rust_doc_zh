#### The Slice Type

切片(Slices)允许你引用集合中一段连续的元素序列，而不是引用整个集合。slice是一种引用，所以它没有所有权(ownership).

这里有一个编程小习题：编写一个函数，传入一个string， 并返回这个string的第一个单词。如果函数在string里没有发现空格，那么整个string就是一个单词，所以整个string应该被返回。

 让我们通过如何写函数签名不用slices，来理解slice将解决的问题：

```rust
fn first_word(s: &String) -> ?
```

`first_word`函数有一个&String作为参数。我们不想要所有权，所以这是可以的。但是我们应该返回什么呢？

我们并没有一个真正获取部分字符串的办法。但是，我们可以返回单词结尾的索引，结尾由一个空格表示。试试如LIsting4-7展示的代码。

Filename: src/main.rs:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Listing 4-7: The `first` function that return a byte index value into the String parameter

因为我们需要通过String的元素一个一个进行检查它是不是一个空格，我们将会转换我们的String为一个byte的数组，使用as_bytes方法：

```rust
let bytes = s.as_bytes();
```

接下来，我们通过使用数组的iter方法创建一个迭代器(iterator)：

```rust
for (i, &item) in bytes.iter().enumberate() {}
```

我们将在第十三章讨论iterators的更多细节。现在，只需要知道iter方法返回一个集合中的每一个元素，而enumerate包装了iter的结果，将这些元素作为元组的一部分返回。从enumerate返回的tuple的第一个元素是索引(index)，第二个元素是一个引用的元素。这比我们自己计算索引(index)更方便一些。

因为enumerate方法返回一个tuple，我们可以用模式(patterns)来结构(destructure)。我们将会在第六章讨论patterns更多细节。在for循环中，我们指定一个pattern有一个`i`接收tuple里的index，以及&item接收tuple里的byte。因为我们从`.iter().enumerate()`得到一个引用，我们在pattern中使用&。

在for循环中，我们通过字节的字面值语法来寻找代表空格的字节。如果找到了一个空格，返回空格的位置。否则，使用s.len()返回字符串的长度：

```rust
        if item == b'' {
            return i;
        }
     }
     s.len()
```

现在我们有一个方法找出字符串中第一个单词的结尾的索引(index)，但是这里有一个问题。我们返回了一个独立的usize，不过它值在&String的上下文中才是有意义的数字。换句话说，因为它是一个与String相分离的值，无法保证将来它仍然有效。思考一下程序Listing4-8中使用Listing4-7的`first_word`。

Filename:src/main.rs

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

```

Listing 4-8: Storing the result from calling the first_word function and then changing the String contents

这个程序编译时不会有任何错误，而且在调用s.clear()之后使用word也不会报错。因为word与s状态完全没有联系，所以word仍然包含值5。可以尝试用5来提取变量s的第一个单词，不过这是有bug的，因为我们将5保存到word之后s的内容已经改变。

我们不得不时刻担心word的索引与s中的数据不再同步，这很啰嗦且容易出错！如果编写一个second_word函数的话，管理索引这件事将更加容易出问题。它的签名看起来像这样：

```
fn second_word(s: &String) -> (usize, usize) {}
```

现在我们要追踪一个开始和一个结束的索引，并且我们甚至有了更多从数据的某个特定状态计算而来的值，但是完全没有与这个状态管理。现在有三个飘忽不定的不相干变量需要保持同步。

幸运地是，Rust为这个问题提供了解决方案：string slices.

#### 字符串切片(String Slices)

一个字符串切片是String的一部分的一个引用，如下：

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

不同于整个String的引用，hello只是引用了String的一部分。由一个额外的[0..5]部分指定。可以使用一个由中括号中的[starting_index..ending_index]指定的range创建一个slice，其中starting_index是slice的第一个位置，ending_index是slice最后一个位置的值。在其内部，slice的数据结构存储了slice的开始位置和长度，长度对应于ending_index减去starting_index的值。所以对于`let world = &s[6..11];`的情况，world将是一个包含指向s索引6的指针和长度5的slice.

![](https://mmbiz.qpic.cn/mmbiz_png/ZsCznicobc9JclqgnwtVOn2GicSjUicIdvUncklzruXEIowcxDSSjYeYDDcV7mfwrmJr1Sr16ukJf1VdzpUBWenKA/0?wx_fmt=png)

Figure 4-6: String slice referring to part of a String

对于Rust的..range语法，如果想要从索引0开始，可以不写两个点之前的值。换句话说，如下两个语句是等价的：

```rust
let s = String::from("hello world");

let slice = &s[0..2];
let slice = &s[..2];
```

以此类推，如果slice包含String的最后一个字节，也可以舍弃尾部的数字。这意味着如下也是等价的：

```rust

#![allow(unused)]
fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
}
```

也可以同时舍弃这两个值来获取整个字符串的的slice。所以如下也是等价的：

```

#![allow(unused)]
fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
}
```

```
注意：字符串slice range的索引必须位于有效的UTF-8字符边界内，如果尝试从一个多字节符的中间位置创建字符串的slice，则程序将会因错误而退出。出于介绍字符串slice的目的，本部分假设值使用ASCII字符集；第八章的"使用字符串存储UTF-8"部分会更加全面地讨论UTF-9处理问题。
```

记住所有这些信息，让我们重新编写first_word返回一个slice。"字符串slice"的类型声明为&str:

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumberate() {
        if item == b‘ ’ {
            return &s[0..i];
        }
    }
    &s[..]
}
```

我们使用Listing 4-7里的方法获取到单词结尾的索引，通过查看第一个空格出现的方式。当我们发现一个空格，我们返回字符串切片，以字符串开头到空格的索引位置。

当我们调用first_word，我们得到一个单独的值，并与底层关联。这个值由一个slice开始位置的引用和slice中元素的数量组成。

second_word函数也可以返回一个slice：

```rust
fn second(s: &String) -> &str {}
```

现在我们有了一个不容易混淆且直观的API了，因为编译器会确保指向String的引用持续有效。还记得Listing4-8中，那个当我们获取第一个单词结尾的索引之后，接着就清除了字符串导致索引就无效的bug吗？那些代码在逻辑上是不正确的，但却没有显示任何直接的错误。问题会在之后尝试对空字符串使用第一个单词的索引时出现。slice就不可能出现这种bug并让我们更早的指定出问题了。使用slice辨别的first_word会抛出一个编译时错误：

Filename: src/main.rs

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

```

如下为编译器错误：

```rust
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 | 
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 | 
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error

```

回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。因为clear需要清空String，它尝试获取一个可变引用。在调用clear之后的println!使用了word中的引用，所以这个不可变的引用在此时必须仍然有效。Rust不仅使得我们的API简单易用，也在编译时就消除一整类的错误!

#### 字符串字面值就是slice(String Literals Are Slices)

回忆一下我们讲过字符串字面值被存储在二进制文件中吗？现在知道slice了，我们就可以正确地理解字符串字面值了：

```rust
let s = "Hello, world!";
```

这里的s的类型是&str：它是一个指向二进制程序特定位置的slice。这也就是为什么字符串字面值是不可变的；&str是一个不可变引用。

#### 字符串slice作为参数(String Slices as Parameters)

知道了能够获取字面值和String的slice后，我们对first_word做了改进，这是它的签名：

```rust
fn first_word(s: &String) -> &str {}
```

而更有经验的Rustacean会编写出示例4-9中的签名，因为它使得可以对String值和&str值使用相同的函数：

```rust
fn first_word(s: &str) -> str {}
```

Listing 4-9: Improving the first_word function by using a String slice for the type s parameter

如果我们有一个字符串slice，我们可以直接传递。如果我们有一个String，我们可以传这个String的slice或者String的引用。这种灵活性利用了deref coercions的优势，这个特性我们将在第十五章的"函数和方法的隐式Deref强制转换"章节中介绍。定义一个获取字符串slice而不是String引用的函数使得我们的API更加通用并且不会丢失任何功能：

Filename: src/main.rs

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

```

#### 其他类型的slice(Other Slices)

String slices,如你所想，是特指字符串，但是也有更加通用的切片类型。

```rust
let a = [1, 2, 3, 4, 5];
```

就跟我们要获取字符串的一部分哪有，我们也会想要引用数组的一部分。我们可以这样做：

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

这个slice的类型是&[i32]。它跟字符串slice的工作方式一样，通过村塾第一个集合元素的引用和一个集合长度。你可以对其他所有集合使用这类slice。第八章讲到vector时会详细讨论这些集合。

#### 总结

所有权、借用和slice这些概念让Rust程序在编译时保证内存安全。Rust语言提供了跟其他系统编程语言相同的方式来控制你使用内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无需额外编写和调试相关的控制代码。

所有权系统影响了Rust中很多其他部分的工作方式，所以我们还会继续讲到这些概念，这将贯穿本书余下的内容。让我们开始第五章吧，来看看如何将多份数据组合进一个struct中。









