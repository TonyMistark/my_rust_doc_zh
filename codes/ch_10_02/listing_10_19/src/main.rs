use std::any::type_name;

fn main() {
    println!("Hello, world!");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    println!("string1 type {}", type_of(&string1));
    println!("string2 type {}", type_of(string2));
    
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);
}


fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
