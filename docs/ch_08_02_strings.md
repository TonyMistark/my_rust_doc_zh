### 使用字符串存储UTF-8编码的文本

我们已经在第四章讲解过String,但是我们将更加深入地研究它们。rust 入门新手们通常会在学习到string的时候会卡住，原因有三个：Rust倾向暴露可能发生的错误，字符串是一种比许多程序员想象中的的更为复杂的数据结构，以及UTF-8。所有这些要素结合起来对于来自其他语言背景的程序员来说就可能显得很困难了。

我们在集合中讨论字符串是因为字符串就是作为字节的集合外加一些方法实现的，当这些字节被解释为文本时，这些方法提供了实用的功能。在这一部分，我们会会将关于String中那些任何集合类型都有的操作，比如：创建，更新，读取。我们也会讨论String与其他集合的不同之处，比如String的索引是很复杂的，由于人和计算机理解String数据方式的不同。

#### 什么是String？

首先我们需要讨论一下属于String的具体意义。Rust的核心语言只有一种字符串类型：字符串slice`str`，它通常以借用的形式出现，`&str`。在第4章中,我们讨论过字符串切片,它们是指向其他地方存储的一些UTF-8编码字符串数据的引用。例如,字符串字面值存储在程序的二进制文件中,因此是字符串切片。

`String`类型是在Rust标准库提供的而不是在核心语言中提供。它是一个可增长，可变的，可持有，UTF-8字符串编码类型。当Rustaceans在Rust中提到`String`的时候，他们可能指的是String或者字符串切片`&str`类型。虽然本节要讨论String，但Rust的标准库广泛使用这两种类型，`String`和字符串切片都是UTF-8编码的。

#### 创建一个新的String

很多与`Vec<T>`相同可用的操作同样在`String`中也可用。因为`String`实际上是作为一个带有一些额外保证，限制和功能的字节vector的封装。其中一个同样作用域`Vec<T>`和`String`函数的例子是用来新建一个实例`new`函数，如Listing8-11示。

```rust
fn main() {
    let mut s = String::new();
}
```

Listing 8-11: Creating a new, empty `String`

这行代码创建了一个空字符串叫做`s`，可以向其中载入数据。我们常常有一些带有初始化数据的需要。为此，我们可以使用`to_string`方法，这对任何类型只要实现了`Display`trait都是可用的，比如字符串字面量(string literals)。Listing 8-12展示了两个例子。

```rust
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
```

Listing 8-12: Using the `to_string`method to create a `String` from a string literal

这段代码创建了一个包含了初始数据的字符串。

我们也可以用函数`String::from`来创建一个String literal。Listing8-13代码实现和Listing8-12中使用`to_string`时等价的。

```rust
fn main() {
    let s = String::from("initial contents");
}
```

Listing 8-13: Using the `String::from` function to create a `String` from a string literal

因为String可以在很多地方被用到，我们可以使用很多不同的通用APIs来生成字符串，因为Rust提供了很多选项。有一些看着是多余的，但是实际上他们都有它实际的用途，`String::from` 和 `to_string`做同样的事情，所以选择那个取决你你的编码风格以及代码可读性。

请记住，字符串是UTF-8编码的，因此我们可以在其中包含任何编码的数据，如Listing 8-14所示.

```rust
fn main() {
    let hello = String::from("你好");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
```

Listing 8-14: Storing greetings in different languages in strings

以上所有这些都是有效`String`值。

#### 更新String

一个`String`长度可以增大并且它的值也可以修改，如果你放入更多的数据，就像修改`Vec<T>` 的内容一样可以修改`String`。另外，你可以方便地使用`+`操作或者`format!`宏来连接`String`

#### 使用`push_str`和`push`来追加一个字符串

我们一个使用`push_str`方法来追加一个String slice使得`String`增长，如Listing 8-15所示

```rust
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
}
```

Listing 8-15: Appending a string slice to a `String` using the `push_str` method

经过这两行代码之后，`s` 就会包含`foobar`。`push_str`方法采用字符串slice，因为我们不需要持有参数的所有权。如Listing 8-16所示，我们想要在`s1`内容中追加`s2`。

```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}
```

Listing 8-16: Using a string slice after appending its contents to a `String`

如果`push_str`拿了`s2`的所有权，我们就不能在最后一行在`println!`中使用了。然而，这不是我们想要的结果。

`push`方法被定义为获取一个单独字符作为参数，并最佳到`String`中。Listing 8-17 使用`push`添加"L"到一个`String`中。

```rust
fn main() {
    let mut s = String::from("LO");
    s.push('L');
}
```

Listing 8-17: Adding one charachter to a `String`value using `push`

以上的代码，最后的结果是,`s`将会包含`LOL`

#### 使用`+`运算符或`format!`宏拼接字符串

常常，你想要将两个已经存在的String拼接起来。一个方式就是使用`+`操作，如Listing 8-18所示。

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}
```

Listing 8-18: Using the `+`operator to combine two `String`values into a new `String`value

`s2`中将会包含`Hello, world!`。 `s1`在相加之后就不再有效的原因，和使用`s2`的引用的原因，与使用`+`运算符时调用的 函数签名有关。`+`运算符使用了`add`函数，这个函数签名如下所示:

```rust
fn add(self, s: &str) -> String {
```

在标准库中，你会看到`add`的定义使用了泛型(generics)和关联(associated)类型。这里我们带入具体类型，那就是当我们用`String`作为参数调用这个方法发生了什么。我们将会在第十章讨论泛型。这个参数签名提供了理解`+`运算那微妙的线索。

首先，`s2`使用了`&`,意味着我们使用第二个字符串的**引用**与第一个字符串相加。这是因为`add`函数的`s`参数：是能将`&str`和`&String`相加，不能将两个`String`值相加。正如`add`的第二个参数所制定的，`&s2`的类型是`&String`而不是`&str`。那么为什么Listing 8-18还能编译呢？

之所以能够在`add`调用中使用`&s2`是因为`&String`可以被强转(coerced)成`&str`。当`add`函数被调用时，Rust使用了一个被称为**Deref强制转换(**deref coercion)的技术，你可以理解为它把`&s2`变成了`&s2[..]`。第十五章会讨论强制转换。因为`add`不会持有`s`参数的所有权，`s2`将仍是一个有效`String`在这个操作之后。

然后，我们可以看到`add`方法拿到了`self`的所有权，因为`self`没有`&`。这意味着Listing 8-18中的`s1`的所有权将被移动到了`add`，并且`s1`将不再有效。所以才`let s3 = s1 + &s2`;可以理解为它将会复制两个String并且创建了一个新的String，这个语句实际上占用了`s1`的所有权，然后复制`s2`并进行追加操作，然后返回结果的所有权。换句话说，它看起来好像生成了很多拷贝，不过实际上并没有：这个实现比拷贝更高效。

如果想级联多个字符串，`+`的行为就显得笨重了:

```rust
fn main() {
    let s1 = String::from("I");
    let s2 = String::from("Love");
    let s3 = String::from("China");

    let s = s1 + "-" + &s2 + "-" + &s3;
}
```

此时，`s`为`I-Love-China`。在有这么多`+`和`"`字符的情况下，很难理解具体发生了什么。对于更为复杂的字符串连接，可以使用`format!`宏：

```rust
fn main() {
    let s1 = String::from("I");
    let s2 = String::from("Love");
    let s3 = String::from("China");

    let s = format!("{s1}-{s2}-{s3}");
}

```

这些代码也会将`s` 设置为`I-Love-China`。 `format!`宏的工作原理与`println!`相同，但是不会打印到屏幕上，它会返回一个`String`内容。这个版本就好理解的多，宏`format!`生成的代码使用引用所以不会获取任何参数的所有权。

#### 索引字符串

在其他编程语言中，通过索引引用字符串中的单个字符是一种有效且常见的操作。但是，在Rust中，如果你尝试使用索引语法访问String的某些部分，则会得到一个错误。如下Listing 8-19为错误示范：

```rust
fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
}
```

Listing 8-19: Attempting to use indexing syntax with a String

这段代码将会出现如下的错误：

```rust
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
  = help: the following other types implement trait `Index<Idx>`:
            <String as Index<RangeFrom<usize>>>
            <String as Index<RangeFull>>
            <String as Index<RangeInclusive<usize>>>
            <String as Index<RangeTo<usize>>>
            <String as Index<RangeToInclusive<usize>>>
            <String as Index<std::ops::Range<usize>>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` due to previous error
```

错误和提示说明了全部问题：Rust的字符串不支持索引。那么接下来的问题是，为什么不支持呢？为了回答这个问题，我们必须先聊聊Rust时如何在内存中储存字符串的。

#### 内部表现

`String`是一个`Vec<u8>`的封装。让我们看一下Listing  8-14中正确编码的UTF-8示例字符串。

```rust
fn main() {
    let hello = String::from("Hola");
}
```

在这里，`len`会是4，这意味着储存字符串`Hola`的`Vec`的长度是四个字节:这里每一个字母的UTF-8编码都占用一个字节。那下面这个例子又如何呢？(注意这个字符串中的首字母是西里尔字母的Ze而不是阿拉伯数字3。)

```rust
fn main() {
    let hello = String::from("السلام عليكم");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("Здравствуйте");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("السلام عليكم");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("Dobrý den");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("Hello");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("שָׁלוֹם");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("नमस्ते");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("こんにちは");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("안녕하세요");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("你好");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("Olá");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("Здравствуйте");
    println!("hello: {}, len: {}", hello, hello.len());
    let hello = String::from("Hola");
    println!("hello: {}, len: {}", hello, hello.len());
}
```

输出结果为：

```rust
hello: السلام عليكم, len: 23
hello: Здравствуйте, len: 24
hello: السلام عليكم, len: 23
hello: Dobrý den, len: 10
hello: Hello, len: 5
hello: שָׁלוֹם, len: 14
hello: नमस्ते, len: 18
hello: こんにちは, len: 15
hello: 안녕하세요, len: 15
hello: 你好, len: 6
hello: Olá, len: 4
hello: Здравствуйте, len: 24
hello: Hola, len: 4
```

这是因为每个Unicode标量值需要两字节存储。因此一个字符串字节的索引并不是对应一个有效的Unicode标量值，作为演示，考虑如下无效的代码：

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

我们已经知道`answer`的第一个字符不是数字`3`。当使用UTF-8编码时，第一个字节(西里尔字母的Ze)`3`的编码是`208`，第二个是`151`，所以`answer`实际上应该是`208`，不过`208`自身并不是一个有效的字母。返回`208`可不是一个请求字符串第一个字母的人所希望看到的，不过它是Rust在字节索引0位置所能提供的唯一数据。用户通常不会想要一个字节值被返回。即使这个字符串只有拉丁字母，如果`&"hello"[0]`是返回紫戒指的有效代码，它也会返回`104`而不是`h`。

#### 字节、标量值和字形簇!天哪!

另外一个关于UTF-8的知识点是，从 Rust 的角度来讲，事实上有三种相关方式可以理解字符串：字节、标量值和字形簇（最接近人们眼中 **字母** 的概念）。

比如这个用梵文书写的印度语单词"नमस्ते"，最终它存储在vector中的`u8`值看起来像这样：

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

这里有18个字节，计算机最终如何存储这些数据。如果我们把他们看做Unicode标量值，它就是Rsut的`char`类型，这些字节看着像如下这样：

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

这里有6个`char`，但是第四个和第六个不是字母：它们是音调，本身没有意义。最后，如果我们把它们看成是字形簇，我们会得到人们所说的组成印度语单词的四个字母。

```rust
["न", "म", "स्", "ते"]
```

Rust提供了不同的方式来解释计算机存储的原始字符串数据，这样每个程序都可以选择它需要的解释，而不管这些数据是什么人类语言。

Rust不允许我们索引String以获取字符的最后一个原因是，索引操作总是需要常数时间(O(1))。但是不能保证String的性能，因为Rust必须从头到尾遍历内容，以确定有多少个有效字符。

#### 字符串slice

字符串中使用索引其实是一个馊主意，因为他没有明确返回String-indexing应该是一个字节(byte)值，字符(char)，字形簇(grapheme cluster)或者字符串切片(string slice)。因此，如果您确实需要使用索引来创建字符串切片，那么Rust会要求您更具体。

你可以使用[]和一个范围来创建包含特定字节的字符串切片，而不是对单个数字使用[]进行索引:

```rust
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
}
```

这里`s`为`&str`，它包含了string `hello`的前四个字节。前面，我们提到每个字符都是2字节，这意味着s将是Зд。如果我们尝试用&hello[0..1]， Rust会在运行时出现报错，就像在vector中访问无效索引一样。

```rust
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:4:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

你应该谨慎地使用范围来创建字符串片，因为这样做可能会导致程序崩溃。

#### 遍历字符串的方法

对字符串片段进行操作的最佳方法是明确说明是需要字符还是字节。对于单个Unicode标量值，使用chars方法。在Зд上调用chars会分离并返回两个char类型的值，您可以遍历结果以访问每个元素

```rust
fn main() {
    for c in "Зд".chars() {
        println!("{c}");
    }
}
```

打印结果为：

```rust
З
д
```

或者，bytes方法返回每个原始字节，这可能适合你的范围：

```rust
fn main() {
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
```

打印结果为：

```rust
208
151
208
180
```

但是一定要记住，有效的Unicode标量值可能由超过1个字节组成。

#### 字符串并不简单

总之，字符串是很复杂的。不同的语言会有不同的选择如何将这种复杂性呈现给程序员。Rust选择将String数据的正确处理作为所有Rust程序的默认行为,这意味着程序员必须在处理UTF-8数据上花更多的心思。这个权衡在其他编程语言中暴露了更多字符串的复杂性,但它可以防止你在开发生命周期的后期处理涉及非ASCII字符的错误。
好消息是标准库基于String和&str类型提供了许多功能来正确处理这些复杂的情况。确保查看文档中有用的方法,比如在字符串中搜索的contains和用另一个字符串替换字符串的一部分replace。
让我们切换到略微简单一点的内容:哈希映射!

