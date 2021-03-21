use blog::Post;

fn main() {
    let mut post = Post::new();
    // State 1. Draft
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    // State 2. Pending Review
    post.request_review();
    assert_eq!("", post.content());
    // State 3. Publishing
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
