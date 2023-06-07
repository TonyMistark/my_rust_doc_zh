trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("pupy")
    }
}

fn main() {
    // error way
    // println!("A baby dog is called a {}", Animal::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
