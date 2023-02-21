fn main() {
    let mut v = vec![100, 32, 57];

    println!("print original v:");
    for i in &mut v {
        println!("{i}");
    }

    // add 50 to each element
    for i in &mut v {
        *i += 50;
    }

    println!("after add 50 print v: ");
    for i in &mut v {
        println!("{i}");
    }
}
