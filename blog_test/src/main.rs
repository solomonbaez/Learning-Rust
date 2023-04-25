
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Rust!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Rust!", post.content());

    println!("I posted: {}", post.content());
}
