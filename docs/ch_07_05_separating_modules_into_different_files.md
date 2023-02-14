## 将模块拆开分成多个文件

到目前为止，所有示例都在一个文件中定义了多个模块。当模块逐渐变大时，你可以将它们的定义移动到一个单独的文件中，以使代码更容易导航。

例如，我们从Listing 7-17包含多个餐厅模块的代码开始。我们会将模块提取到各自的文件中，而不是将所有模块都定义到crate中。在这里，crate根文件是src/lib.rs，不过这个过程也适用于crate根文件是src/main.rs的二进制crate.

首先将`front_of_house`模块提取到其自己的文件中。删除`front_of_house`模块的大括号中的代码，只留下`mod front_of_house;`声明，这样src/lib.rs会包含如Listing7-21所示的代码。注意直到创建示例 7-22中的src/front_of_house.rs文件之前代码都不能编译。

Src/lib.rs

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
		hosting::add_to_waitlist();
}
```

Listing 7-21: Declaring the `fron_of_house`module  whose body will be in src/front_of_house.rs

然后，然后把花括号内的代码放入src/fron_of_house.rs的新文件中，如Listing 7-22所示。因为编译器找到了crate根中名叫`front_of_house`的模块声明，它就知道去搜寻这个文件。

src/fron_of_house.rs

```rust
pub mod hosting {
		pub fn add_to_waitlist() {}
}
```

Listing 7-22: Definitions inside the `front_of_house`module in src/front_of_house.rs

注意你只需在模块树中的某处使用一次`mod`声明就一块架在这个文件。一旦编译器知道了这个文件是项目的一部分(并且通过`mode`语句的位置知道了代码在模块树中的位置)，项目中的其他文件应该使用其所声明的位置的路径来引用那个文件的代码，这在“7.3: Paths for Referring to an Item in the Module Tree”有讲到。换句话说，`mod`不是你可能会在其他编程语言中看到的"include"操作。

然后，我们提取`hosting`模块到自己的文件中。这个过程会有所不同，因为`hosting`是`front_of_house`的子模块而不是根模块。我们将`hosting`的文件放在模块树中它的父级模块同名的目录中，在这里是src/front_of_house/

为了移动`hosting`，修改src/front_of_house.rs使之仅包含`hosting`模块的声明。

src/front_of_house.rs

```rust
pub mod hosting;
```

接着我们创建一个src/front_of_house文件夹，并常见一个文件hosting.rs来放定义在`hosting`模块里面的代码：

src/front_of_house/hosting.rs

```rust
pub fn add_to_waitlist() {}
```

如果将hosting.rs放在src目录，编译器会认为`hosting`模块中的hosting.rs的代码声明于crate根，而不是声明为`front_of_house`的子模块。编译器所遵循的哪些文件对应哪些模块的代码的规则，意味着目录和文件更接近于模块树。

#### 备选文件路径

目前为止我们介绍了Rsut编译器所常用的文件路径；不过一种更老的文件路径也仍然是支持的。对于声明在crate根的`front_of_house`模块，编译器会在如下位置查找模块代码：

* src/front_of_house.rs (我们已经介绍的)
* src/front_of_house/mod.rs(更老的风格，仍支持路径)

对于一个叫做`hosting`的模块，是`front_of_house`的子模块，编译器将会查看模块的代码在如下路径：

* src/front_of_house/hosting.rs (我们已经介绍的)
* src/front_of_house/hosting/mod.rs(更老的风格，仍然支持)

如果你在同一个模块使用两种风格，你将会得到一个编译错误。在一个项目中不同的模块之间混用两种风格是允许的，但是在项目中对他人造成困惑。



我们已经将各个模块的代码移到不同的文件了，同时模块树依旧相同。`eat_at_restaurant`中的函数调用也无需修改继续保持有效，即便其定义存在于不同的文件中。这个技巧让你可以在模块代码增长时，将它们移动到新文件中。

注意，src/lib.rs中的`pub use crate::front_of_house::hosting`语句是没有改变的，在文件作为crate的一部分而编译时，`use`不会有任何影响。`mod`关键字声明了模块，Rust会在与模块同名的文件中查找模块代码。

#### 总结

Rust提供了将包分成多个crate，将crate分成模块，以及通过指定绝对或者相对路径从一个模块引用另一个模块中定义的项的方式。你可以指定绝对路径或者相对路径。这个路径可以通过`use`声明引入到作用域，所以你可以在多次使用时可以使用更短的路径。模块定义的代码默认是私有的，不过可以选择增加`pub`关键字使其定义为公有。

接下来，我们来看一些标准库提供的集合数据类型，你可以利用它们写出更漂亮整洁的代码。