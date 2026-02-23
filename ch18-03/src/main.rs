use ch18_03::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("hello");

    let post = post.request_review();

    let mut post = post.reject();
    post.add_text("world");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("helloworld", post.content());

    // assert_eq!("", post.content());

    // post.approve();
    // assert_eq!("hello", post.content());
}
