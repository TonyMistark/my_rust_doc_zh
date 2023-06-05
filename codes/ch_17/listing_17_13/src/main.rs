use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    println!("first approve: {}", post.content());
    post.approve();
    println!("second approve: {}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());
}
