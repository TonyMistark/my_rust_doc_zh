fn main() {
    unsafe fn dangerous() {
        println!("I am a unsafe fn");
    }

    unsafe {
        dangerous();
    }
}
