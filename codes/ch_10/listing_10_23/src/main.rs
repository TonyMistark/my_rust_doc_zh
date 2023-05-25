fn main() {
    println!("Hello, world!");
    let string1 = String::from("logn string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = logest(string1.as_str(), string2.as_str());
    }
    println!("The logest string is {}", result);

}

fn logest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
