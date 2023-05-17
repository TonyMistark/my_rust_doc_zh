use std::println;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    println!("Hello, world!");
    let novel = String::from("Call me Ishmael. Some years ago..");
    let firt_sentence = novel.split('.').next().expect("Could not find a '.'");
    let ie = ImportantExcerpt {
        part: firt_sentence,
    };
    println!("ie.part: {}", ie.part);
}
