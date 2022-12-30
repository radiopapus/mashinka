#![allow(clippy::must_use_candidate)]
use crate::grow::post::Post;
use std::collections::HashMap;

use crate::grow::{
    ISO8601_DATE_TIME_FORMAT, KEYWORDS_DELIMITER, KEY_VALUE_DELIMITER, LF, META_DELIMITER,
};

/// Преобразует строку в `DraftPost`. Строка должна удовлетворять формату grow записи. Например
/// key: value
///---
/// content
pub fn deserialize(draft_content: &str) -> HashMap<&str, &str> {
    let (meta, content) = draft_content
        .trim()
        .split_once(META_DELIMITER)
        .unwrap_or_else(|| panic!("Meta and content should be delimited by {}", META_DELIMITER));

    let meta_lines: Vec<&str> = meta.trim().split(LF).collect();

    let meta_key_values = meta_lines.iter().map(|line| {
        line.split_once(KEY_VALUE_DELIMITER)
            .expect("Check meta key value delimiter")
    });

    let mut hashmap = HashMap::new();

    for (key, value) in meta_key_values {
        hashmap.insert(key, value);
    }

    hashmap.insert("content", content);

    hashmap
}

/// Преобразует Post в форматированную строку для grow записи.
/// title здесь это идентификатор - slug, который будет использован для системы перевода
pub fn serialize_with_template(post: &Post, template: String) -> String {
    let published_date = post
        .published_date_time
        .format(ISO8601_DATE_TIME_FORMAT)
        .to_string();

    let template_tuple = [
        ("title", post.slug.clone()),
        ("author", post.author.clone()),
        ("description", post.description.clone()),
        ("image", "/static/images/default.png".to_string()),
        ("lang", post.lang.to_string()),
        ("slug", post.slug.clone()),
        ("content", post.draft_content.clone()),
        ("publish_date", published_date),
        ("keywords", post.keywords.join(KEYWORDS_DELIMITER)),
    ];

    let key_values = HashMap::from(template_tuple.map(|(k, v)| (k, v)));

    process_template(template, key_values)
}

pub fn process_template(mut content: String, hashmap: HashMap<&str, String>) -> String {
    for (key, value) in hashmap {
        content = content.replace(&format!("[{key}]"), &value);
    }
    content
}
