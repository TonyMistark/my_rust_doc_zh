fn main() {
    println!("Hello, world!");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("{:?}", &r1);
    println!("{:?}", &r2);
}
