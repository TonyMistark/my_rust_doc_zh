---
title: Ch 07.03:创建公有的结构体和枚举(Making Structs and Enums Public)
date: 2022-12-31 13:35
tags: Rust
layout: Rust
---
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
