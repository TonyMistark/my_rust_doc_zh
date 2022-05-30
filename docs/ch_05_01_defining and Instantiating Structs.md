#### Defining and Instantiating Structs

Structs 类似于tuple，在"The Tuple Type"模块讨论过，都可以包含多个相关联的值。也可以有不同类型的值在struct中。但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定货访问实例中的值。

定义一个结构体(struct)，我们需要输入关键字struct以及结构体名。结构体的名字需要描述它所组合的数据的意义。接着，在大括号中，定义每一部分数据的名字和类型，我们称为字段(field)。例如Listing 5-1展示了一个结构体存储了用户的信息：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {}
```

Listing 5-1: A User struct definition

一旦定义了结构体，我们为每一个字段指定一个值来创建这个结构体的一个实例。我们创建一个实例通过花括号包括key:value的键值对，key是结构体声明的字段名，value是对应的字段需要存储的值。我们不必按照声明的字段顺序来指定字段的值。换句话说，结构体定义像是一个普通的类型模板，实例在这个模板中放入特定的数据来创建这个类型的值。例如我们可以声明一个特定的user如Listing 5-2:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

Listing 5-2: Creating an instance of the User struct

为了从结构体中获取某个特定的值，可以使用点号"."。如果我们想获得这个用户的email，我们可以使用user1.email在使用这个值的地方。如果实例是可变的，我们可以通过点改变特定的字段的值。Listing5-3展示了如何修改email这个字段的值。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Listing 5-3:Changing the value in the email field of a User instance

注意，所有的实例必须是可变的；Rust不允许只将某些字段设置为可变。任何表达式，我们可以构建一个新的实例，将其作为函数体最后一个表达式，以隐式方式返回该实例。

Listing 5-4展示了一个`build_user` 函数，它返回了一个`user`的实例并赋值email和username。active值为true，sign_in_count值为1。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```

Listing 5-4:A build_user function that takes an email and username and return a User instance

使用与结构体字段名字作为函数的参数名是有意义的，但是重复email和username字段名显得有点单调。如果结构体有更多的字段，重复每一个字段的名字会更加烦人。幸运地是，有方便简洁的方法。

#### 使用字段初始化简写语法(Using the Field Init Shorthand)

因为结构体字段参数的名字是明确的，像Listing 5-4，我们可以可以用字段初始化简写语法重写`build_user`，因此它表现上和前面写法一样明确，但是不需要重复email和username,如Listing 5-5所展示。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```

Listing 5-5 A `build_user` function that uses field init shorthand because the `email` and `username` parameters hanve the same name as struct fields

这里，我们创建了一个`User`结构体的新的实例。我们想要通过 `build_user`函数的 参数`email`设置结构体字段`email`的值。因为`email`字段和`email` 参数有相同的名字，我们只需要写`email`而不是`email: email`。

#### 使用结构体更新语法从其他实例创建实例(Creating Instances From Other Instances With Struct Update Syntax)

创建一个实例包含一个结构体的大部分值经常是很有用的，但对一些值进行修改。你可以只用update语法做到这个。

首先，在Listing 5-6中，我们展示了如何创建一个新的`User`实例user2,使用update 语法。我们给email设置一个新的值，否则使用了Listing 5-2同样的值。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Listing 5-6:Creating a new `User`instance one of the values from user1

使用结构体更新语法，我们可以以很少的代码完成同样的变动，如Listing 5-7所示。语法`..`指定未显式设置其余字段的值为给出的这个实例的值。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Listing 5-7的代码也创建了一个实例`user2`， 除了email值不同之外的字段都和user1相同。`..user1`必须排在最后面，以指定剩余的字段的值都取自于`user1`对应的字段，同时，我们可以选择任意数量的字段，而不用考虑这些字段在结构体里面定义的顺序。

注意，结构体更新语法使用`=`像一个分配行为；这是因为它移动这个数据，就像我们在"Says Varibles and Data Interact : Move"部分所看到的一样。在这个例子中，我们可以不再使用`user1`在创建`user2`之后，因为在`user1`的`username`的`String`已经被移动到`user2`中了。如果我们给`user2`的`email`和`username`都赋予新的`String`值，从而只使用`user1`的`active`和`sign_in_count`值，那么user1在创建user2后仍然有效。active和sign_in_count的类型是实现Copy trait的类型，所以我们在"Stack-Only Data: Copy"部分讨论的行为同样适用。

#### 使用没有名字字段的元组结构体来创建不同的类型(Using Tuple Structs without Named Fields to Create Different Types)

也可以定义与元组(在第三章讨论过)类似的结构体，称为元组结构体(tuple structs)。元组结构体有着结构体名称提供含义，但没有具体的字段名，只有字段的类型。当你想给整个元组取一个名字，并使元组成为与其它元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。

要定义元组结构体，以`struct`关键字和结构体名开头后跟元组中的类型。例如，厦门的两个分别叫做Color和Point元组结构体的定义和用法：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

注意，black和origin值的类型不同，因为它们是不同的元组结构体的示例。你定义的每一个结构体有其自己的类型，即使结构体重的字段有着相同的类型。例如，一个获取Color类型参数的函数不能接受Point作为参数，即使这两个类型都由三个i32值组成。在其他方面，元组结构体实例类似于元组：可以将其结构为单独的部分，也可以使用`.`后跟索引来访问单独的值，等等。

#### 没有任何字段的类单元结构体(Unit-Like Structs Without Any Fields)

我们可以定义一个没有任何字段的结构体，它们称为类单元结构体(Unit-like structs)因为它们类似于`()`，即"元组类型(The Tuple Type)"一节中听到过unit类型。类单元结构体常常在你想要在某个类型上实现trait但是不需要在类型中存储数据的时候发挥作用。我们将在第十章介绍trait.下面是一个声明和实例化一个名为`AlwaysEqual`的unit结构体例子。

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

要定义`AlwaysEqual`，我们使用struct关键字，我们想要的名称，然后是一个分号。不需要花括号或者圆括号。然后，我们可以以类似的方式在subject变量中获得`AlwaysEqual`的实例：使用我们定义的名称，不需要任何花括号或者圆括号。想象一下，我们将实现这个类型的行为，即每个实例始终等于每一个其他类型的实例，也许是为了获得一个已知的结果以便进行测试。我们不需要任何数据来实现这种行为，你将在第十章中，看到如何定义特性并在任何类型上实现它们，包括类单元结构体。

#### 结构体数据的所有权(Ownership of Struct Data)

在Listing 5-1中的`User`结构体定义中，我们使用了自身拥有所有权的String类型而不是&str字符串slice类型。这是一个有意为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话数据也是有效的。可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上生命周期(lifetimes)，这是一个第十章会讨论的Rust功能。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不是指定生命周期将是无效的，比如这样：

filename: src/main.rs

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

编译器会控诉它需要生命周期标识符：

```rust
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` due to 2 previous errors=
```

第十章会讲到如何修复这个问题以便在结构体中存储引用，不过现在，我们使用像String这类拥有所有权的类型来替代&str这样的引用以修正这个错误。