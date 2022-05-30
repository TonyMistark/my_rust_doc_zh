#### 什么是所有权(What is Ownership) ?

所有权(Ownership)是控制Rus程序如何管理内存的一些规则。所有程序在运行时必须管理它们使用一个计算机的内存的方式。一些语言有垃圾回收机制，就是不断地查看在程序运行中不再使用的内存；另外一些语言，程序设计者必须显式地分配和释放内存。Rust是第三种方法：内存被一个所有权系统管理，编译器在编译时会根据一系列的规则进行检查。如果任何一个规则被违反，程序将不会编译。不会有一个所有权(ownership)的特性会使得你的程序在运行时变慢。

因为ownership对于很多程序设计者来说是一个新的概念，需要一些时间来习惯它。好消息是，Rust和ownership的规则会使得你更有经验，你会发现你自然地开发出安全且有效的代码变得更加容易。保持！

当你理解了ownership，你将有个坚实的基础来理解那些使得Rust显得独一无二的特性功能。在本章，你会通过一些聚焦于一个非常通用的数据结构：string的示例来学习ownership。

<u>栈和堆(The Stack and the heap)</u>
<u>很多语言不会要求你思考栈和堆。但是在像Rust这样的系统语言中，一个值是否在栈(stack)或者堆(heap)上会影响到这个语言的表现，并且为什么你必须做出一些明确的决定。ownership的一部分就是描述栈和堆，稍后在这章，所以这里只是一个简短的预先准备的解释。</u>

<u>堆和栈都是代码在运行时可用的内存，但是它们是不同的数据结构。栈存储值是有序的，获取和移出值是相反的顺序。被称为后进(last in), 先出(first out)。想一下一叠盘子：当你添加更多的盘子，你把它们放在最上面，而当你需要一个盘子的时候，你也是从最上面拿一个。从中间或者底部添加或者移走一个盘子就不好操作了！！添加数据被称为进栈(pushing onto the stack), 而移出数据被称为出栈(poping of the stack)。栈中的所有的数据都必须占有已知的固定大小。一个不知道大小的数据在编译时间或者大小可能会被改变的必须存放在堆上。</u>

<u>堆是缺乏组织的：当你向堆放入数据，你需要一个确定的空间。内存分配器(memory allocator)会在堆上找一个足够大的空白块，将它标记为已使用，并返回一个表示该位置地址的指针(pointer)。这个过程被称为堆上分配(allocating on the heap)，有时简写为分配(allocating)。往栈上面放入不被认为是分配(allocating)。因为指针的大小是已知且固定的，你可以把指针存储在栈上，但是当你想要真实的值，要通过指针去拿。想象一下坐在餐馆。当你进入的时候，你要说出你们一起有几位，工作人员寻找一个可以适合你们每一个人的桌子并带你到那里。如果有人来晚了，他们可以问你坐在哪儿，然后找到你。</u>

<u>入栈比在堆上分配要更快，因为堆上分配的时候永远必须寻找一个存储新数据的地方；入栈永远是在栈顶。相对来说，在堆上分配空间需要更多的工作，因为分配器必须首先找到一个足够大的空间存放数据，然后执行记录(bookeeping)为下一个分配做准备。</u>

<u>访问堆的数据比访问栈的数据更慢，因为必须通过指针才能找到对应存放数据的位置。当代的处理器如果在内存中跳转越少就越快。继续类比，想象一个餐厅服务员要接受很多桌的点菜事物。最有效的方法是在一个桌上拿到所有的点菜需求之后再移动到下一桌。在A桌点一个菜，然后又去B桌点一个菜，然后又回到A桌，然后又回到B桌这样将会处理效率特别慢。同样地，一个处理器在处理的数据彼此比较近的时候(比如在栈上)比较远的时候(比如在堆上)能更好地工作。在堆上分配大量的空间也可能消耗时间。</u>

<u>当你的代码调用一个函数，值被传入函数(包含潜在可能的在堆上的指针数据)并且函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。</u>

<u>跟踪哪些代码用到了哪些在堆上的数据，尽量减少堆上的重复数据，以及清理堆上不再使用的数据确保不会消耗空间，这些问题正是ownership系统要处理的。一旦你理解了ownership，你就不需要经常去思考堆和栈了，但是理解owership主要的目的就是管理堆数据，能够帮助解释ownership为什么要以这种方式工作。</u>

#### 所有权规则(Owership Rules)

首先，让我们看看owership的规则。记住这些规则，当我们举例说明的时候：

* Rust中的每一个值都有一个被称为所有者(owner)的变量。
* 值在任何时刻有且只有一个所有者(owner)。
* 当所有者离开作用域，这个值将会被丢掉。

#### 变量作用域(Variable Scope)

现在我们已经掌握了基本的Rust语法，我们将不会在之后的例子中包含fn main() {} 代码，所以如果你是一路跟过来的，必须手动将之后的例子代码手动放入main函数中。结果就是，我们的例子将会更加简洁，聚焦到实际的细节中，而不是构建代码。

作为第一个ownership的例子，我们将了解变量的作用域(scope)。作用域就是程序中一个项目有效的范围。如下例子：

```
let s = "hello";
```

变量s指向了一个字符串字面量(string literal)，字符串变量是硬编码到我们程序的文本中的。这个变量是从它被声明的这一刻开始到当前作用域结束之前都有效。Listing 4-1附带的注释展示了变量在哪个区域是有效的。

```rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

Listing 4-1:A variable and the scope in which it is valid

换句话说， 这里有两个重要的时间点：

* 当s进入作用域，它是有效的。
* 它直到出作用域之前都是有效的。

此时，作用域和变量有效之间的关系似乎和其他语言是类似的。现在，我们将在此理解的基础之上引入String类型。

#### 字符串类型(The String Type)

为了说明ownership的规则，我们需要一个比在第三章数据类型部分更复杂的数据类型。前面介绍的类型都是已知大小的，可以存储在栈(stack)上并且在它们出作用域之后出栈，如果代码的另外一部分需要在不同的作用域中使用相同的值，可以快速简单地复制它们，来创建一个新的独立的实例。但是，我们想看看一个存储在堆(heap)上的数据并且探索Rust怎么知道清理这个数据，而且String类型是一个非常好的例子。

我们专注于String于ownership关联的部分。这种特性也适用于其他的数据类型，无论它们是有标准库提供的还是由你创建的，我们将在第八章深入讨论String。

我们已经见过字符串字面量，一个字符串变量的值是硬编码入我们的程序，String字面量是很方便的，不过它们并不适用使用文本的每一种场景。原因之一就是它们是不可变的。另一个就是不是所有的字符串都可以在我们编写代码的时候知道它的大小：例如，如果我们想要接受用户的输入并存储它？对这种情况，Rust有第二种String类型。这个类型管理被分配到堆上的数据，所以能够存储在编译时未知大小的文本。可以使用from函数基于字符串字面量来创建String，如下：

```rust
let s = String::from("hello");
```

这两冒号::是运算符，允许将特定的函数置于String类型的命名空间(namespace)下，而不需要使用类似string_from这样的名字。第五章的方法语法("Method Syntax")部分会着重将接这个语法而且在第七章的"路径用于引用模块树中的项"中也会降到模块的命名空间。

可以修改此类型字符串:

```rust
let mut s String::from("hello");
s.push_str(", world!");    // push_str() 在字符串后追加字面量
println!("{}", s);    // 将打印 `hello, world!`
```

所以，这里有什么不同？为什么我们可以String可以可变但是字面量不可以？区别在于两个类型对内存的处理上。

#### 内存与分配(Memory and Allocation)

在string literal的情况中，我们在编译时就知道其内容，所以文本是直接硬编码到最终的执行文件中。这就是为什么string litrals是快速且高效。但是这些属性只是因为string litral的不可变性。不幸的是，我们不能为了每一个在编译时大小还未知的文本而将一块内存放入二进制文件中，并且它的大小还有可能随着程序的运行而改变。

对于String类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：

* 必须在运行时向内存分配器(memory allocator)请求内存。
* 需要一个当我们处理完String时将内存返回给分配器的方法。

第一部分由我们完成：当调用String::from时，它的实现(implementation)请求其所需的内存。这在编程语言中是非常通用的。

然而，第二部分实现起来各有不同。在有垃圾回收(garbage collector，GC)的编程语言中，GC追踪并清理已经不再使用的内存，我们并不需要去考虑它。在大多数编程语言中没有GC，确认不在使用并且的内存，调用代码显式释放就是我们的责任了，就跟请求内存的时候一样。能正确地做到这一点，历史经验告诉我们是一个很困难的问题。如果我们忘记，我们就会浪费内存，如果我们释放太早，我们就会有一个不可用的变量。如果我们释放两次，那也是一个bug。我们需要显式地一对一的分配和释放。

Rust采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。下面是Listing4-1使用一个String而不是string litral的版本:

```
fn main() {
    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    }                                  // 此作用域已结束，
                                       // s 不再有效
}
```

这是一个将String需要的内存返回给分配器的很自然的位置：当s离开作用域的时候。当变量离开作用域，Rust为我们调用一个特殊函数。这个函数叫做drop，在这里String的作者可以放着代码来返回内存。Rust在花括号结束时}自动调用drop。

注意：在C++中，这种item在生命周期结束时释放资源的模式有时被称作 资源获取即初始化(Resource Acqurisition Is Initialization(RAII))。如果你使用过RAII模式的话，应该对Rust的drop函数并不陌生。

这个模式对编写Rust代码的方式有着深远的影响。现在它看起来很简单，不过在复杂的场景下代码的行为可能是不可预测的，比如当有多个变量使用在堆上分配的内存时。现在让我们探索一些场景。

#### 变量与数据交互的方式：移动(Ways Variables and Data interact:Move)

在Rust中多个变量可以采取不同的方式与同一个数据进行交互。让我们看一个例子，Listing 4-2:

```rust
let x = 5;
let y = x;
```

我们大概可以猜测这是在干什么：“绑定一个值5给x；然后拷贝x的值并绑定给y。”我们现在又两个变量，x和y并且都等于5。这确实是在发生的事情，因为整数是简单的已知的，固定大小的值，并且这两个5都入栈了。

现在让我们看一下String版本：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

这看起来很相似，所以我们假设他们是相同的工作方式：第二行将会对s1做一个拷贝并绑定到s2。但事实并非如此；

看看Figure 4-1以了解String的底层会发生什么。String由三部分组成，图左侧：一个指针指向保存字符串内容的内存，一个长度(len)和一个容量(capacity)。这个一组数据存储在栈上。图右侧则是堆上存放内容的内存部分。

![Figure 4-1](https://kaisery.github.io/trpl-zh-cn/img/trpl04-01.svg)

Figure 4-1:Representation in memory of a string holding the value "hello" bound to s1

长度(len)表示String的内容当前使用了多少字节的内存。容量(captacity)是String从分配器那里总共获取了多少字节的内存。长度与容量的区别是很重要的，不过在当前上下文中并不重要，所以现在可以忽略容量。

当我们将s1赋值给s2，String的数据被赋值了，这意味着我们从栈上拷贝了它的指针，长度和容量。我们并没有赋值指针指向的堆上的数据。换句话说，内存中数据的表现如Figure 4-2。

![Figure 4-2](https://doc.rust-lang.org/book/img/trpl04-02.svg)

Figure 4-2: Representation in memory of the variable s2 that has a copy of the pointer, length and capacity of s1

其表现不像Figure 4-3,如果Rust也赋值堆数据，内存就会编程这样。如果Rust这样做，那么久运行时性能而言，如果堆上的数据很大，那么s2=s1操作对性能影长就会非常大。

![Figure 4-3](https://kaisery.github.io/trpl-zh-cn/img/trpl04-03.svg)

Figure 4-3：Another possibility for what s2 = s1 might do if Rust copied the heap data as well

早些时候，我们说当一个变量走出作用域，Rust会自动调用drop函数并清理这个变量在堆上的内存。但是Figure 4-2显示两个数据指针指向了相同的地址。这是一个问题：当s2和s1走出作用域，它们将都会尝试释放相同的内存。这就是我们前面已经提到的已知的两次释放内存的错并且是内存安全错误之一。两次释放内存会导致内存污染，它可能会导致潜在的内存安全漏洞。

为了确保内存安全，在`let s2 = s1;`这行之后，Rust认为s1不再是有效的了。因此，当走出s1的作用域，Rust不需要释放任何东西。

看看在创建s2之后，尝试使用s1会发生什么？它是不会正常工作的：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

你会得到一个如下的错误，因为Rust组织你使用一个无效的引用：

```rust
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

如果你在使用其他语言听过浅拷贝(shallow copy)和深拷贝(deep copy)这两个术语，这个拷贝指针，长度，容量但不拷贝数据的概念听起来就像是浅拷贝(shallow copy)。但是因为Rust也会使用一个变量无效，而不是将其浅拷贝，所以被称为移动(move)。在这个例子中，我们说是s1被移动到s2。所以实际的表现如Figure 4-4:

![Figure 4-4](https://doc.rust-lang.org/book/img/trpl04-04.svg)

Figure 4-4: Representation in memory after s1 has been invalidated

这样就解决了我们的问题！因为只有s2是有效的，当其离开作用域，它就释放自己的内存，完毕。

另外，这里还隐含了一个设计选择：Rust永远不会自动创建数据的“深拷贝”。因此，任何自动的复制可以被认为对运行时性能影响最小。

#### 变量与数据的交互方式：克隆(Ways Variable and Data Interact: Clone)

如果我们想要深拷贝String的数据，不仅仅是栈数据，我们可以使用一个叫做clone的通用函数。我们将在第五章讨论这个方法的语法，但是因为这个函数在很多编程语言中都是一个很通用的函数，你之前也许见过。

如下有一个clone函数的示例：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

这段代码能正常运行，并且明确产生Figure 4-3中的行为，这里堆上的数据确实被复制了。

当出现clone调用时，你知道一些特定的代码执行而且这些代码可能想当消耗资源。你很容易能感觉到一些不寻常的事情正在发生。

#### 只在栈上的数据：拷贝(Stack-Only Data: Copy)

这里还有一个没有提到的小窍门。这些代码使用整型并且是有效的，他们是Listing 4-2的部分内容：

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

但是这段代码似乎与我们刚刚学到的内容相矛盾：没有调用clone，但是x仍然是有效的，没有被移动到y。

原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实是很快的。这意味着没有利益在创建变量y后使得x无效。换句话说，这里没有深浅拷贝的区别，所以这里调用clone并不会与通常的浅拷贝有什么不同，我们可以不用管它。

Rust有一个叫做Copy trait的特殊解释，可以用在类似整数这样的存储在栈上的类型上(第十章详细讲解trait)。如果一个类型实现了Copy Trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。Rust不允许自身或者其他任何部分实现了Drop trait的类型使用Copy trait。如果我们对其值离开作用域时需要特殊处理的类型使用了copy 注解，将会出现一个编译时错误。要学习如何为你的类型添加copy注解实现该trait，请阅读附录C"Deribable Traits"。

所以那些类型实现了Copy Trait呢？可以查看给定类型的文档来确认，不过作为一个通用规则，任何一组简单标量值的组合都可以实现copy，任何不需要分配内存或者某种形式资源的类型都可以实现Copy。如下是一些Copy的类型：

* 所有整数类型，比如u32
* 布尔类型，bool，它的值是true 和 false
* 所有浮点数类型，比如 f64
* 字符类型，char
* 元组，当且仅当其包含的类型也都实现了Copy的时候。比如：(i32, i32)实现了copy，但是(i32,String)就没有。

#### 所有权和函数(Ownership and Functions)

将值传递给函数在语义上与给变量赋值相似。响函数传递值可能会移动或者复制，就像赋值语句一样。Listing 4-3使用注释展示变量合适进入和离开作用域：

Filename: src/main.rs

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

```

Listing 4-3:Functions with ownership and scope annotated

当我们尝试调用takes_ownership 之后再想要使用s时，Rust会抛出编译时错误。这些静态检查使我们免于犯错。试试在main函数中添加使用s和x的代码来看看那里能使用他们，以及ownership规则会在那里阻止我们这样做。

#### 返回值与作用域(Return Values and Scope)

返回值也可以转移所有权。Listing 4-4展示了一个示例，与Listing4-3一样带有类似的注释。

Filename: src/main.rs

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Listing 4-4: Transferring ownership of return values

变量的所有权总是遵循相同的模式：将赋值给另一个变量时移动它。当持有堆中数据的变量离开作用域时，其值将通过drop被清理，除非数据被移动为另一个变量所有。

虽然这样是可以的，但是在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都穿进去再返回来就有点烦人了，除此之外，我们可能想返回函数中产生的一些数据。

我们可以使用元组来返回多个值，如Listing 4-5。

Filename：src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

Listing 4-5: Returning ownership of parameters

但是，对于一个应该是普通的概念来说，这太多的仪式和大量的工作。幸运的是，Rust有一个特性，可以在不用转移所有权的情况下使用值，它叫做引用(references)。

接下来将讲"引用和借用(References and Borrowing)"

























