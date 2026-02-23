use ch18_03::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("hello");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("hello", post.content());
}
