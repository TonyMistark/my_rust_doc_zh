use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        // function requires argument type to outlive `'static` (rustc E0373)
        println!("Here'a a vector: {:?}", v);
    });

    handle.join().unwrap();
}
