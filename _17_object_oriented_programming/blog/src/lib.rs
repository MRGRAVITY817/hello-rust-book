pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  // Constructor
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text)
  }
  // Ensuring content is empty
  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(&self)
  }
  // Request review
  pub fn request_review(&mut self) {
    // Because when we use the state, it's considered to turn 'old'
    // so we renew it, by changing the ownership
    if let Some(s) = self.state.take() {
      // take() method takes away the current state's ownership to 's'
      self.state = Some(s.request_review())
    }
  }
  // Changes the behavior of the content
  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  // fn content -> ""
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
  // fn content -> ""
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}
