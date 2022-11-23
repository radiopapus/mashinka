struct PostMeta {
    preview_image_url: String,
    order: u32,
    lang: String,
    slug: String,
    keywords: [String],
    description: String
}

struct Author {
    first_name: String,
    last_name: String
}

struct Post {
    title: String,
    author: Author,
    timestamp: u32,
    content: String,
    meta: PostMeta
}