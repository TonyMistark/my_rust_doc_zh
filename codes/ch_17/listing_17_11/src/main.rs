use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_test("I ate a salad for lunch today");
    assert_ne!("", post.content());

    post.request_review();
    assert_ne!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content())
}
