fn main() {
    // this pattern will always match, so the `if let` is useless
    if let x = 5 {
        println!("{}", x);
    }
}
