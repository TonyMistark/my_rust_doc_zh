fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
    // 如果尝试对同一闭包使用不同类型则就会得到类型错误。
}
