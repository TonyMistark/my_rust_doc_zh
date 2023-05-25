fn main() {
    println!("Hello, world!");
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest_with_an_anouncement(
        s1.as_str(),
        s2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}


use std::fmt::Display;

fn longest_with_an_anouncement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
