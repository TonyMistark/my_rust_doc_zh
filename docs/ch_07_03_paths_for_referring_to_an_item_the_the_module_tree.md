## 引用模块项目的路径

来展示一下Rust是如何在模块树里面找到一个项(item)，我们使用路径的方式，就像在文件系统中一样。如果我们要调用一个函数，我们需要知道它的路径。

路径有两种形式：

* 绝对路径(absolute path): 是完整的路径从crate root开始；以crate名或者字面量值crate开头。
* 相对路径(relative path): 从当前模块开始以`self`、`super`或者当前模块的标识符开头。

绝对路径和相对路径都是遵循通过(`::`)来分隔的。

回到Listing 7-1，我们如何调用`add_to_waitlist`函数呢？还是同样的问题，`add_to_waitlist`函数的path是什么？Listing 7-3包含Listing 7-1中一些模块和函数被移除。

我们在create跟定义了一个函数`eat_at_restaurant`，并在其中展示两种方式来调用`add_to_waitlist`。

`eat_at_restaurant`函数是我们crate库的一个公共API，所以我们使用`pub`关键字来标记它。在"Exposing Paths with the `pub` Keyword"部分，我们将会更加详细讲解`pub`。

src/lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Listing 7-3: Calling the `add_to_waitlist` function using absolute and relative paths

第一次我们调用`add_to_waitlist`函数在`eat_at_restaurant`中，我们使用了一个绝对路径。`add_to_waitlist`函数和`eat_at_restuarant`函数被定义在相同的crate，这意味着我们可以使用`crate`关键字开始一个绝对路径。然后，我们将每一个连续的模块包括在内，直到找到我们的`add_to_waitlist`路径。你可以想象一下一个文件系统也是相同的结构：我们指定路径`/front_of_house/hosting/add_to_waitlist`来执行`add_to_waitlist`程序。我们使用`crate`从crate根开始就类似于在shell中使用`/`从文件系统根开始。

第二种方式，我们在`eat_at_restaurant`中调用`add_to_waitlist`，使用的是相对路径。这个路径以`front_of_house`为起始，从这个模块在模块树中，与`eat_at_restaurant`定义在同一层级。与之等价的文件系统路径就是`front_of_house/hosting/add_to_waitlist`。以名称为起始，意味着该路径是相对路径。

选择使用相对路径还是绝对路径，还是取决于你的项目。取决于你是更倾向于将项的定义代码与使用该项的代码分开来移动，还是一起移动。举例，如果我们要将`front_of_house`模块和`eat_at_restaurant`函数移动到一个名为`customer_experience`的模块中，我们需要更新`add_to_waitlist`的绝对路径，但是相对路径还是可用的。然后，如果我们要将`eat_at_restaurant`函数单独移动到一个名为`dining`的模块中，还是可以使用原本的绝对路径来调用`add_to_waitlist`，但是相对路径必须要更新。我们更倾向于使用绝对路径，因为把代码定义和调用各自独立地移动是更常见的。

让我们试着编译一下Listing 7-3，并查明为何不能编译！Listing 7-4展示了这个错误。

```rust
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

Listing 7-4: Compiler errors from building the code in Listing 7-3

这个错误信息表明：`hosting`是私有的。换句话说，我们的`hosting`模块和`add_to_waitlist`函数的路径是正确的，但是Rust不让我们使用它们，因为它不能访问私有部分。

模块不仅对于你组织代码有用。他们还定义了Rust的私有性边界(privacy boundary)：这个边界线不允许外部代码了解，调用和依赖被封装的实现细节。所以，如果你希望创建一个私有函数或者结构体，你可以将其放入模块。

Rust中默认所有项(函数，方法，结构体，枚举，模块和常量)都是私有的。父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。这是因为模块封装并隐藏了他们的实现细节，但是子模块可以看到他们定义的上下文。继续拿参观作为比喻，把私有性规则想象成餐馆的后台办公室：餐馆内的事物对餐厅顾客来说是不可知的，但办公室经历可以洞悉其经营的餐厅并在其中做任何事情。

Rust选择以这种方式来实现模块系统功能，因此默认隐藏内部实现细节。这样一来，你就知道可以更改内部代码的哪些部分而不会破坏外部代码。你可以通过使用`pub`关键字来创建公共项，使子模块的内部部分暴露给上级模块。

#### 使用`pub`关键字暴露路径(Exposing Paths with the pub Keyword)

让我们回头看一下Listing 7-4的错误，它告诉我们`hosting`模块是私有的。我们想让父模块中的`eat_at_restaurant`函数可以访问子模块中的`add_to_waitlist`函数，因此我们使用`pub`关键字来标记`hosting`模块，如Listing7-5所示:

src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Listing 7-5: Declaring the `hosting`module as `pub` to use it from `eat_at_restaurant`

不幸地，Listing 7-5中的代码仍然有一个错误，如Listing 7-6所示：

```rust
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> src/lib.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

Listing 7-6: Compiler errors from building the code in Listing 7-5

发生了什么？在`mod hosting`的前面加上`pub`关键字将这个模块变成公有的。伴随着这种变化，如果我们可以访问`front_of_house`，那么我们也可以访问`hosting`。但是`hosting`的内容(contents)仍然是私有的；这表明模块公有并不使其内容也是公有的。模块上的`pub`关键字只允许其父模块引用它。因为模块是一个容器(Container)，仅仅通过将模块设置为公有，我们可以做的事情并不多；我们还需要进一步选择将模块中的一个或者多个项设为公有的。

Listing 7-6中的错误，`add_to_waitlist`是私有函数。隐私规则适用于结构体，枚举，函数，和方法以及模块(structs, enums, functions,and methods as well as modules)。

让我也在`add_to_waitlist`函数前添加`pub`关键字使它成为共有函数，如Listing 7-7

src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Listing 7-7: Adding the `pub` keyword to `mod hosting` and `fn add_to_waitlist` lets us call the function from `eat_at_restaurant

现在，代码将可以编译了！让我们看看绝对路径和相对路径，并根据私有性规则，再检查一下为什么增加了`pub`关键字使得我们可以在`add_to_waitlist` 中调用这些路径。

在绝对路径，我们从`crate`，也就是crate根开始。然后crate根中定义了`fron_of_house`模块。`front_of_house`模块不是公有的，不过因为`eat_at_restaurant`函数与`front_of_house`定义于同一模块中(即，`eat_at_restaurant`和`front_of_house` 是兄弟)，我们可以从`eat_at_restaurant`中引用`front_of_house`。接下来是使用`pub` 标记的`hosting`模块。我们可以访问`hosting` 的父模块所以可以访问`hosting`。最后，`add_to_waitlist` 函数被标记为`pub`，我们可以访问其父模块，所以这个函数调用是有效的！

在相对路径，其逻辑与绝对路径相同，除了第一步：不同于从crate根开始，路径从`front_of_house`开始。`front_of_house`模块与`eat_at_restaurant`定义于同一模块，所以从`eat_at_restaurant`中开始定义该模块相对路径是有效的。接下来因为`hosting`和`add_to_waitlist`被标记为`pub`，路径其余的部分也是有效的，因此函数调用也是有效的！

如果你计划共享你的library crate，以便其他项目也可以使用你的代码，你的公有API是你与库使用者的合同，它决定了他们如何与你的代码交互。关于管理公有API的更改，有许多注意事项，以使人们更容易依赖你的crate。这些讨论内容不在这本书的讨论范围；如果你对这个话题感兴趣，看这里"The Rust API Guideline"(https://rust-lang.github.io/api-guidelines/)

##### 带有二进制文件和库的包最佳实践(Best Practices for Packages with a Binary and a Library)

`我们提到一个包(package)可以包含一个src/main.rs二进制crate根和一个src/lib.rs library crate 根，而且这两个crates有默认的包名。通常，采用这种既包含库(library)又包含二进制crate的形式，有足够的代码来启动一个可执行文件，该可执行文件调用带有library crate。这使得其他项目可以从包提供的大部分功能中受益，因为library crate的代码可以共享。`

`模块树应该被定义在src/lib.rs。然后，任何公有项可以在二进制crate中使用，以包的名称作为起始路径。二进制crate成为一个library crate的用户，就像完全地外部crate使用这个library crate：它只能使用公共API。这将帮助你设计一个好的API；不仅你是作者，你也是客户！ `

`在 Chapter 12中, 我们将用一个命令行程序来演示这种组织练习，该程序将包含二进制crate和一个library crate。`

#### 使用super起始的相对路径(Starting Relative Paths with super)

我们可以构造从父模块开始的相对路径，而不是从当前模块或者crate根，通过在路径前面使用`super`。这就像文件系统的路径语法`..`一样。使用`super`允许我们引用一个项，我们已知的父模块，当模块与父模块密切相关时，这种方法可以更容易地重新排列模块树。但是父模块也可能在某一天会被移动到模块树的别的地方。

考虑一下Listing 7-8的代码，该代码模拟了这样一种情况：厨师修正了错误的订单，并亲自将其端给客户。`fix_incorrect_oder`函数定义在`back_of_house`模块并调用函数`deliver_order`定义在父模块，通过指定的`super`开始的路径指向`deliver_order`：

src/lib.rs

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

Listing 7-8: Calling a function using a relative path starting with `super`

`fix_incorrect_order`函数在`back_of_house` 模块中，所以我们可以使用`super`来指向`back_of_house`父模块，这个实例中就是根`crate`。从这里开始，我们找到`deliver_order`。成功！我们认为`back_of_house`模块和`serve_oder`函数之间可能具有某种关联关系，并且，如果我们要重新组织这个crate的模块树，需要一起移动它们。因此我们使用`super`，这样一来，如果这些代码被移动到了其他模块，我们只需要更新很少的代码。

#### 创建公有的结构体和枚举(Making Structs and Enums Public)

我们也可以使用`pub`来指定结构体和枚举为公有，但是这里有一些额外的使用详情需要注意。如果我们在使用`pub`定义一个结构体，我们使得这个结构体为公有，但是结构体的字段将仍是私有的。我们可以视情况而定让每一个字段成为公有或者私有。在Listing 7-9中，我们定义了一个公有的`back_of_house::Breakfast`结构体并且`toast`为公有字段，而`seasonal_fruit`为私有字段。该模型以餐厅为例，顾客可以选择随餐面包的类型，但是主厨会根据当季水果和库存来搭配什么水果。可选择的水果变化很快，所以顾客不能选择水果，甚至不能看到他们能得到的水果。

src/lib.rs

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

Listing 7-9: A struct with some public fields and some private fields

因为`toast`字段在`back_of_house::Breakfast`结构体内是公有的，在`eat_at_restaurant`中我们可以编写并且读到`toast`字段使用点(.)号。注意，我们不能在`eat_at_restaurant`中使用`seasonal_fruit`字段，因为`seasonal_fruit`字段是私有的。尝试取消注释修改季节水果字段的值，看看会得到什么错误。

同样，注意因为`back_of_house::Breakfast`有一个私有字段，这个结构体需要提供一个公有的关联函数来构造一个`Breakfast`实例(这里是名为`summer`的函数)。如果`Breakfast`没有这样一个函数，我们就不能在`eat_at_restaurant`中创建`Breakfast`实例，因为我们不能在`eat_at_rest_aurant`中给`seasonal_fruit`私有字段设置值。

相反地，如果我们设置枚举为公有，它所有的字段都是公有的。我们只需要在`enum`关键字前面加上`pub`即可，如Listing 7-10所示：

src/lib.rs

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Listing 7-10: Designating an enum as public makes all its variants public

因为我们设置了`Appetizer`为公有枚举，我们可以在`eat_at_restaurant`中使用`Soup`和`Salad`成员变量。

除非设置枚举的变量为公开的，否则枚举就没什么用了；在每种情况下都必须使用`pub`来声明所有枚举变量，这将是很恼人的设计，所以默认情况下枚举的所有变量都是公有的。结构体在字段为私有的情况通常是有用的，所以结构体字段默认遵循所有内容为私有的普遍规则，除非使用`pub`声明公有。

这里还涉及`pub`另外一个情况我们没有讨论到，那就是我们最后要讲的模块功能：`use`关键字。我们将单独介绍`use`，然后展示如何结合`pub`和`use`起来使用。
