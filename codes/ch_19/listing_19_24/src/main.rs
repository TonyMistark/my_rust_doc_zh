fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| ())
    }
}
