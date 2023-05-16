fn main() {
    println!("Hello, world!");

    another_function();

    let number = 6;
    let num = number % 4;
    println!("{number} % 4 = {num}");

    play_if_else();
    play_while();


}

fn another_function() {
    println!("Another function");
}

fn play_if_else() {
    let condition = true;

    let _number = if condition {
        5
    } else {
        6
    };
    // let numner = 5;
    // println!("numner: {numner}");
}

fn play_while() {
    let mut number = 3;
    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }
    println!("LIFEOFFF!");
}
