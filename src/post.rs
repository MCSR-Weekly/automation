#[derive(Debug, Clone)]
pub(crate) struct Post {
    pub(crate) rich: RichPost,
    pub(crate) text: TextPost,
}

#[derive(Debug, Clone)]
pub(crate) struct RichPost {
    pub(crate) message: String,
    pub(crate) embed_title: String,
    pub(crate) embed_desc: String,
    pub(crate) url: String,
}

#[derive(Debug, Clone)]
pub(crate) struct TextPost {
    pub(crate) content: String,
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
