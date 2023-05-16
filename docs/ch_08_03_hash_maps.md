### 使用Hash Map存储键值对

最后介绍的常用集合是*hash map*。`HashMap<K, V>`类型存储了一个键类型`K`对应一个值类型`V`的映射。它通过一个*hashing function*来实现映射，决定如何将键和值放入内存中。很多编程语言支持这种数据结构，不过通常有不同的名字：hash, map, object, hash table,  dictionary, 或者associative array等等。

Hash maps是很有用的，当你想查看一个数据，但是不想用像vector一样用索引，当你用一个键key(可以是任何类型)。例如，在一个游戏中，你可以持续追踪每个队伍的分数，就可以用hash map，key可以为队伍的名字，对应的值为分数。给一个队伍名，你就可以得到这个队伍的分数。

本章我们会介绍hash map的基本API,不过还有更多吸引人的功能隐藏于标准库在`HashMap<K, V>`上定义的函数中。详情请查看标准库来了解更多。

#### 新建一个Hash Map

创建空hash map其中一个方法时使用`new`并通过`insert`添加成员。Listing 8-20，我们持续追踪*Blue队*和*Yellow队*两队的分数。Blue对初始为10分，Yellow队初始为50分。

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

Listing 8-20: Create a new hash map and inserting some keys and values

注意，我们首先需要使用`use`从标准库的`collections`引入`HashMap`.在我们的三个常见集合中，这个集合是最不常用的，因此它不包括在预加载中自动引入作用域中。hash map也没有得到标准库的支持;例如，没有内置的宏来构造它们。

像vectors一样，hash map将它们的数据存储在堆上。这个`HashMap`key的类型为`String`，value的类型为`i32`。像vectors一样，hash map是同质的(*homogeneous*):也就是说所有的key类型必须相同，并且所有的values值的类型也必须相同。

#### 访问Hash Map中的值

我们可以用`get`方法通过提供key来获取对应的值，如Listing 8-21所示：

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}
```

Listing 8-21: Accessing the score for the Blue team stored in the hash map

这里，`score`会为Blue队关联分数10。`get`方法会返回一个`Option<&V>`；如果key没有对应的值，`get`方法会返回`None`。程序通过调用`copied`来处理`Option`，会得到一个`Option<i32>`而不是一个`Option<&i32>`，然后如果通过key没有拿到`score`的话使用`unwrap_or`方法设置`score`为0。

我们可以用与vector类似的方式来遍历hash map中的每一个键值对，也就是for循环：

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
```

这段代码的输出为：

```rust
Yellow: 50
Blue: 10
```

#### Hash Maps和所有权

对于像`i32`这样实现了`Copy`trait的类型，其值可以copy进hash map。对于`String`这样拥有所有权的值，其值将被移动而hash map会成为这些值的所有权。如Listing 8-22所示：

```rust
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

Listing 8-22: Showing that keys and values are owned by the hash map once thery're inserted

在调用`insert`将`field_name`和`field_value`移动到hash map之后，我们就不能再使用这两个变量了。

如果我们将值的引用插入到hash map中，这些值不会被移动到hash map中。引用所指向的值必须至少在hash map有效的时间内有效。我们将会在第十章的["Validating References with Lifetimes"](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes)部分详细讲解。

#### 更新Hash Maps

尽管键和值对的数量是可以增长的, 每个唯一键一次只能有一个与之关联的值。

当你想要更改hash map数据时，你必须决定如何处理键已经赋值的情况。你可以用新值替换旧值，完全忽略旧值。或者你可以把旧值和新值结合起来。让我们来看看如何做到这一点!

#### 覆盖(Overwriting)值

如果我们在hash map中插入一个键和一个值，然后用不同的值插入相同的键, 与该键关联的值将被替换，虽然Listing 8-23的代码调用`insert`两次，但是hash map也将只存一个key/value键值对。因为我们插入两次Blue队的key。

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}
```

Listing 8-23: Replacing a value stored with a particular key

这段代码将打印{"Blue": 25}。原来的值10已被覆盖。

#### 只在Key没有值的情况下插入值

通常检查一个特定的键是否已经存在于带有值的hash map中，然后采取以下操作:如果键确实存在于hash map中，则现有的值应保持原样。如果键不存在，则插入一个值。

Hash map为此有一个特殊的API，称为`entry`，它将你想要检查的键key作为参数。`entry`方法的返回值是一个名为`Entry`的枚举，它表示一个可能存在也可能不存在的值。假设我们想要检查Yellow队的键是否有与之关联的值。如果不存在，我们想要插入值50，对于Blue队也是一样。使用`entry`API，代码Listing 8-24所示。

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```

Listing 8-24: Using the `entry` method to only insert if the key does not already a value

定义`Entry`上的`or_insert`方法返回对应的`Entry`键(如果该键存在)的值的可变引用。如果不存在，插入参数里的值并返回这个新值的可变引用。。这比编写自己的逻辑要简明的多，另外也与借用检查器结合的很好。

运行Listing 8-24的代码会输出`{"Yellow": 50, "Blue": 10}`。对`entry`的第一次调用将为Yellow队插入值为50的键，因为Yellow队还没有值。对`entry`的第二次调用不会改变哈希映射，因为Blue队的值已经是10了。

#### 根据旧值更新一个值

Hash map的另一个常见用例是查找键的值，然后根据旧值更新它。如Listing 8-25所示，计算每个单词在某些文本中出现多少次的代码。我们使用一个以单词为键的hash map，并增加该值，以跟踪我们看到该单词的次数。如果这是我们第一次看到一个单词，我们将首先插入值0。

```rust
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
```

Listing 8-25: Counting occurrences of words using a hash map that stores words and counts

这段代码将会输出：`{"world": 2, "hello": 1, "wonderful": 1}`。你自己运行也许会看到不同的顺序，因为hash map是无序的。

`split_whitespace`方法返回一个迭代器，遍历文本中由空格分隔的子切片。`or_insert`方法返回一个对指定键值的可变引用`(&mut V)`。这里我们将可变引用存储在count变量中，因此为了对该值赋值，必须首先使用星号(`*`)解除对count的引用。可变引用在for循环结束时将超出作用域，因此所有这些更改都是安全的，并且是借用规则允许的。

#### Hashing 函数

默认情况下，`HashMap`使用一种叫做SipHash的hashing 函数，它可以抵御涉及[hash table](https://en.wikipedia.org/wiki/SipHash)的(Denial of Service, Dos)攻击。然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。如果性能检测显示默认hash函数太慢，你可以切换为其他不同的hash函数。hasher是一个实现了`Builder`trait的类型。第十章会讨论trait和如何实现它们。你并不需要从头开始实现自己的hasher；

#### 总结

vector、字符串和哈希 map 会在你的程序需要储存、访问和修改数据时帮助你。这里有一些你应该能够解决的练习问题:

- 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
- 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
- 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。

标准库 API 文档中描述的这些类型的方法将有助于你进行这些练习！

我们已经开始接触可能会有失败操作的复杂程序了，这也意味着接下来是一个了解错误处理的绝佳时机！