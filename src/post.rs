#[derive(Debug, Clone)]
pub struct Post {
    pub rich: RichPost,
    pub text: TextPost,
}

#[derive(Debug, Clone)]
pub struct RichPost {
    pub message: String,
    pub embed_title: String,
    pub embed_desc: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct TextPost {
    pub content: String,
}

// --

impl From<Post> for RichPost {
    fn from(post: Post) -> Self {
        post.rich
    }
}

impl From<Post> for TextPost {
    fn from(post: Post) -> Self {
        post.text
    }
}
