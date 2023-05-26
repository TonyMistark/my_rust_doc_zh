struct CustomerPointer {
    data: String,
}

impl Drop for CustomerPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerPointer with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomerPointer {
        data: String::from("my stuff"),
    };
    let d = CustomerPointer {
        data: String::from("other stuff"),
    };

    println!("CustomerPointers created.");
}
