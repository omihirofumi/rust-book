use ch18_03::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("hello");

    let review = post.request_review();

    let publised = review.approve();

    // // already moved
    // review.approve();

    assert_eq!("hello", publised.content());

    // assert_eq!("", post.content());

    // post.approve();
    // assert_eq!("hello", post.content());
}
