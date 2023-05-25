use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The lagrest member is x = {}", self.x);
        } else {
            println!("The lagrest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p = Pair {
        x: 66,
        y: 55,
    };
    p.cmp_display();


    let pp = Pair::new(10, 20);
    pp.cmp_display();
    
    println!("Hello, world!");
}
