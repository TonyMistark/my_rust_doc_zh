## 用Vector存储列表的值

第一个集合类型我们来看`Vec<T>`，也称为vector。vector允许我们在单个数据结构上存储超过一个值，它在内存中彼此相邻地排列所有的值。Vector只能存储类型相同的值。当你有一些值的时候，它很有用，例如文件中的每一行的文本内容，或者购物车中的商品的价格。

#### 创建一个Vector(Create a New Vector)

为了创建一个空的vector，我们调用`Vec::new`函数，如Listing 8-1所示：

```rust
fn main() {
  let v: Vec<i32> = Vec::new();
}
```

Listing 8-1: Create a new, empty vector to hold values of type `i32`

注意，这里我们加了类型声明。因为我们不能创建一个任意值的vector，Rust不知道我们想要存储什么类型的的值。这一点很重要。vector是使用泛型实现的，第十章会涉及到如何对你自己的类型使用它们。现在，只需要知道`Vec<T>`类型是由标准库提供的，可以存储任何类型。当我们创建一个vector来存放一个特定类型，我们可以指定类型用<>。在Listing 8-1中，我们告诉Rust，`Vec<T>`要存放的元素的是`i32`类型。

更多的时候，你会用一个初始化的值创建一个`Vec<T>`，并且Rust会推断出你要存放的值的类型。所以，很少需要做类型声明。为了方便，Rust提供了`vec!`宏，这个宏会根据我们提供的值来创建一个新的vector。Listing 8-2创建一个`Vec<i32>`来存放值1，2和3。整数类型是`i32`，因为整数的默认类型就是`i32`，在第三章"Data Types"有提及。

```Rust
fn main() {
    let v = vec![1, 2, 3];
}
```

Listing 8-2: Create a new vector containing values

因为给了初始值`i32`，Rust可以推导出`v`的类型为`Vec<i32>`，并且类型注解是不需要的。下一个我们来了解如何修改一个vector。

#### 更新一个Vector(Updating a Vector)

更新一个Vector，添加一个元素，我们可以使用`push`方法，如Listing 8-3:

```rust
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

Listing 8-3: Using the `push` method to add values to a vector

和任何变量一样，如果我们想改变它的值，我们需要使用`mut`关键字来使它可变，这已经在第三章讨论过了。这里我们用到的值全部是`i32`类型，并且Rust可以推导这些数据类型，我们不需要`Vec<i32>`的类型标注。

#### 读Vector的元素(Reading Elements of Vectors)

 这里有两个方式可以引用到存储在Vector里的值：通过下标或者使用`get`方法。在接下来的例子中，为了更加清晰，我们注释了从这些函数返回的值的类型。

Listing 8-4所示，两种方式访问vector的值，用索引语法或者`get`方法。

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
```

注意这里一些小细节。我们使用下标`2`来得到第三个元素是因为vector元素的下标是从0开始计数。使用`&`和`[]`我们会得到一个Vector对应下标的元素的一个引用。当我们使用`get`方法传入下标为参数，我们会得到一个`Option<&T>`类型，它可以与`match`一起使用。

Rust提供了两种引用元素的方法的原因是当尝试使用现有元素类型之外的索引值时可以选择让程序如何运行，举个例子，让我们看看使用这个技术，尝试在有5个元素的Vector中访问第100位置的元素会发生什么，如Listing 8-5所示：

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
```

Listing 8-5: Attempting to access the element at index 100 in a vector containing five elements

当运行这段代码，第一个`[]`方法会导致程序panic,因为它引用了一个不存在的元素，如果您希望程序在试图访问超出vector结尾的元素时崩溃，则最好使用此方法。

当`get`方法传入一个超出vector范围的下标，它会返回`None`并且不会报错。如果在正常情况下偶尔会访问超出vector范围的元素，则可以使用此方法。你的代码将需要有逻辑去处理`Some(&element)`或者`None`，如第六章讨论的那样。例如，索引可能来源于用户输入的数字。如果他们不慎输入一个过大的数字那么程序就会得到`None`值，你可以告诉用户当前vector元素的数量并再请求他们输入一个有效的值。这就比因为输入错误而使程序崩溃有好多了。

一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则(第四章讲到)来确定vector内容的这个引用和任何其他引用保持有效。回忆一下不能再相同作用域中同时存在可变和不可变引用的规则。这个规则适用于Listing 8-6，当我们获取了Vector的第一个元素的不可变引用并尝试在Vector末尾增加一个元素的时候，如果尝试在函数的后面引用这个元素是行不通的：

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
}
```

Listing 8-6: Attempting to add an element to a vector while holding a reference to an item

编译错误信息：

```rust
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {first}");
  |                                      ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error
```

Listing 8-6 中的代码看起来应该能够运行：为什么第一个元素的引用关心vector结尾的变化呢？不能这么做的原因是由于vector的工作方式：在Vector的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了释放的内存。借用规则阻止程序陷入这种状况。

#### 遍历vector中的元素(Iterating over the Values in a Vector)

依次访问Vector中的元素，我们将遍历所有元素，而不是通过下标索引一次访问一个元素。Listing 8-7 展示如何使用`for`循环来获得Vector的每一个元素，得到不可变的`i32`引用，并打印它们。

```rust
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}
```

Listing 8-7: Printing each element in a vector by iterating over the elements using a `for` loop

我们也可以迭代可变vector中的每个元素的可变引用，以方便对所有元素进行修改。`for`循环在Listing 8-8中给每个元素加`50`:

```rust
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
```

Listing 8-8: Iterating over mutable references to elements in a vector

为了修改可变引用所指向的值，在使用`+=`运算之前必须使用引用符号(`*`)获取`i`中的值。第十五章["Following the Pointer to the Value with the Deference Opterator"](https://doc.rust-lang.org/book/ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator)将会详细讲解

因为借用(borrow)检查器的规则，迭代一个vector，可变或者不可变，都是安全的。如果我们在Listing8-7和Listing 8-8的`for`循环中插入或者移除元素，将会得到一个编译错误，类似于Listing 8-6的错误。for循环所持有的对vector的引用可以防止同时修改整个vector。

#### 使用枚举来存储多种类型(Using an Enum to Store Multiple Types)

Vectors 只能存储相同类型的数据，这就很不方便了，实际应用中肯定有需要存储不同类型的情况。幸运地是，枚举的成员都被定义为相同的枚举类型，因此我们需要一种类型来表示不同类型的元素时，我们可以定义和使用枚举。

例如，假设我们想从电子表格中的一行中获取值，该行中的一些列包含整数，一些浮点数，和一些字符串。我们可以定义一个枚举，它的成员可以持有不同类型的值，并且所有的枚举成员将被认为是相同的类型：就是这个枚举类型。然后我们可以创建一个Vector来保存那个enum,最终保存不同的类型。我们在Listing 8-9中演示：

```rust
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

Listing 8-9: Defining an `enum` to store values of different types in vector

Rust 需要在编译时间知道Vector中的元素都是什么类型，因为它要明确地知道需要多少的内存来将每个元素存储到堆上。我们还必须明确这个vector中允许的类型。如果Rust允许vector可以保存任何类型，在对Vector元素操作时，可能会导致操作执行错误。使用枚举外加`match`意味着Rust能在编译时就保证总是会处理所有可能的情况，如第六章讲的那样。

如果在编写程序时不能确切无遗地知道运行时会存储进行Vector的所有类型，枚举就行不通了。相反，你可以使用trait对象，第十七章会讲到。

我们现在了解了一些使用vector的常见的方式，请一定去看标准库中`Vec`定义的很多其他实用方法的[API文档](https://doc.rust-lang.org/std/vec/struct.Vec.html) 。例如，除了`push`之外还有一个`pop`方法，它会移除并返回vector的最有一个元素。

#### 丢弃vector 时也会丢弃其所有元素(Dropping a Vector Drops Its Elements)

像其他数据结构一样，当一个vector离开了它的作用域，它就会被释放，如Listing 8-10所示：

```rust
fn main() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
```

Listing 8-10: Showing where the vector and its elements are droped

当vector被删除时，它的所有内容也会被删除，这意味着它所持有的整数将被清除。借用检查器确保仅在vector本身有效时才使用对向量内容的任何引用。



接下来，我们将学习下一个集合类型：`String`