#### Method Syntax

方法(method)类似于函数：我们声明方法是用fn后跟方法名，可以有参数并且有一个返回值，它们可以包含来自任何其他地方的一些代码。不像函数(function), 方法定义在结构体上下文内(或者一个枚举(enum)或者trait object, 我们将会在第6章和第17章分别讲述)，并且它们的第一个参数永远是`self`，它代表实例的结构体的方法被调用。

#### 定义方法(Defining Methods)

让我们把前面实现的获取一个Rectangle实例作为参数的area函数，改为一个定义于Rectangle结构体上的area方法，如Listing 5-13所示：

Finename: src/main.rs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

Listing 5-13: Defining an `area`method on eht Rectangle struct

为了使函数定于于Rectangle的上下文中，我们开始了一个`impl`块(impl是implementation的缩写),这个impl块中的所有内容都将于Rectangle类型相关。接着讲area函数移动到impl大括号中，并将签名中的第一个(在这里也是唯一一个)参数和函数体中其他地方的对应参数改成self。然后在main中我们先前调用area方法并传递rect1作为参数的地方，改成使用方法语法(method syntax)在Rectangle实例上调用area方法。方法语法获取一个实例并加上一个点号，后跟方法名，圆括号以及任何参数。

在area的签名中，使用`&self`来代替`rectangle: &Rectangle`，&self实际上是`self: &Self`的缩写。在一个impl块中，`Self`类型是impl块的类型的别名。方法的第一个参数必须有一个名为self的Self类型的参数，所以Rust让你在第一个参数位置上只用self这个名字来缩写。注意，我们仍然需要在self前面使用&来表示这个方法借用了Self实例，就像我们在`rectangle: &Rectangle`中做的那样。方法可以选择获得self的所有权，或像我们这里一样不可变地借用self，或者可变地借用self，就跟其他参数一样。

这里选择&self的理由跟在函数版本中使用&Rectangle是相同的：我们并不想获取所有权，只希望能读取结构体中的数据，而不是写入。如果想要在方法中改变调用方法的实例，需要将第一个参数改为`&mut self`。通过仅仅使用self作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将self转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始实例。

使用方法替代函数，除了使用方法和不需要在每个函数签名中重复self的类型之外，其主要好处在于组织性。我们将某个类型实例能做的所有事情一起放入impl块中，而不是让将来的用户在我们的库中到处找Rectangle的功能。

请注意：我们可以选择将方法的名称与结构体中的一个字段相同。例如，我们可以在Rectangle上定义一个方法，并命名为width：

Filename: src/main.rs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

这里，我们选择创建一个width方法，如果实例的width的值大于0就返回true，否则返回false。我们可以出于任何目的，在同名的方法中使用同名的字段。在main中，当我们在rect1.width后面加上括号时，Rust知道我们知道是方法width。当我们不是用括号时，Rust知道我们指的是字段width。

通常，但是并不总是如此，与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。这样的方法被称为getters，Rust并不像其他一些语言那样为结构字段自动实现它们。Getters很有用，因为你可以把字段变成私有的，但方法是公有的，这样就可以把字段的只读访问作为该类型公共API的一部分。我们将在第七章中讨论什么是私有和公有，以及如何将一个字段或者方法定为公有或私有。

#### 运算符'->'到哪里去了？

在C/C++语言中，有两种不同的运算符来调用方法：. 直接在对象上调用方法，而 -> 在一个对象的指针上调用方法，这时需要先解引用(dereference)指针。换句话说，如果object是一个指针，那么object->something()就像(*object).something一样。

Rust并没有一个与->等效的运算符；相反，Rust有一个叫自动引用和解引用(automatic referencing and dereferencing)的功能。方法调用是Rust中少数几个拥有这种行为的地方。

它是这样工作的：当使用object.something()调用方法时，Rust会自动为object添加&，&mut，或 * 以便使object与方法签名匹配。也就是说，这些代码是等价的：

```rust

#![allow(unused)]
fn main() {
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
   fn distance(&self, other: &Point) -> f64 {
       let x_squared = f64::powi(other.x - self.x, 2);
       let y_squared = f64::powi(other.y - self.y, 2);

       f64::sqrt(x_squared + y_squared)
   }
}
let p1 = Point { x: 0.0, y: 0.0 };
let p2 = Point { x: 5.0, y: 6.5 };
p1.distance(&p2);
(&p1).distance(&p2);
}
```

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

第一行看起来简洁得多。这种自动引用的行为之所以有效，是因为方法有一个明确的接受者，即self的类型。在给出接收者和方法名的前提下，Rust可以明确地计算出方法是仅仅读取(&self)，做出修改(&mut self)或者是获取所有权(self)。事实上，Rust对方法接受者的隐式借用让所有权在实践中更友好。

#### 方法带更多参数(Methods with More Parameters)

让我们练习使用方法在Rectangle结构体上实现第二个方法。现在，我们想要一个Rectangle实例获取另一个Rectangle实例，如果self(第一个Rectangle)能完全包含第二个矩形则返回true，否则返回false。一旦我们定义了can_hold方法，就可以编写程序如Listing 5-4所示：

filename: src/main.rs

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

Listing 5-14: Using the as-yet-unwritten `can_hold` method

并且期待输出如下，因为因为rect2的两个维度都小于rect1，而rect3比rect1要宽：

```rust
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

因为我们想定义一个方法，所以它应该位于impl Rectangle中。方法名是can_hold，并且它会获取另一个Rectangle的不可变借用为参数。通过观察调用方法的代码可以看出参数是什么类型的：rect1.can_hold(&rect2)传入了`&rect2`，它是一个Rectangle的实例rect2的不可变借用。这是可以理解的，因为我们只需要读取rect2(而不是写入，这意味着我们需要一个不可变借用)，而且希望main保持rect2的所有权，这样就可以在调用这个方法后继续使用它。can_hold的返回值是一个布尔值，其实现会分别检查self的宽高是否大于另外一个Rectangle实例。让我们添加can_hold方法到impl块中，如Listing 5-15:

filename: src/main.rs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

Listing 5-15: Implementing the can_hold method on Rectangle than takes another Rectangle instance as a parameter

运行这段代码，我们就会得到我们预期的输出。在方法签名中，可以在self后增加多个参数，而且这些参数就像函数中的参数一样工作。

#### 关联函数(Associatie Fuctions)

所有在impl块中定义的函数被称为`关联函数(associated functions)`,因为它们与impl后面命名的类型相关。我们可以定义不以self为第一个参数的关联函数(因此不是方法),因为它们并不作用于一个结构体的实例。我们已经使用了一个这样的函数：在`String`类型上定义的`String::from`函数。

不是方法的关联函数经常被用作返回一个结构体新实例的构造函数，例如我们可以提供一个关联函数，它接受一个维度参数并且同时作为宽高，这样可以轻松地创建一个正方形Rectangle而不必制定两次同样的值：

Filename: src/main.rs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
}
```

使用结构体名和`::`语法来调用这个关联函数：比如`let sq = Rectangle::square(3)`。这个方法位于结构体的命名空间中: `::`语法用于关联函数和模块创建的命名空间。第七章会讲到模块。

#### 多个impl块(Multiple impl Blocks)

每一个结构体都被允许有多个impl块。例如与Listing 5-15等价的代码Listing 5-16，每个方法都在自己的impl块内。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

Listing 5-16: Rewriting Listing 5-15using multiple `impl`blocks

这里没有为什么要把它们分开到不同的impl块里，主要是为了示例讲解。但是这个语法是对的。我们会在第十章看到多impl块的好处，并且要讨论一般的类型和traits。

#### 总结(Summary)

结构体让你创建自定义的类型在你的domain中。通过使用结构体，你可以将数据关联起来，使得它看起来更加清晰。在impl块内，你可以定义函数并关联你的类型，而且方法是一个很好的关联方式，它让你的结构体有了可以指定它特定行为的能力。

但是结构体不是唯一的方式来创建自定义类型：让我们转到Rust的枚举(enum)特性来，为你的工具箱再添加一个工具。