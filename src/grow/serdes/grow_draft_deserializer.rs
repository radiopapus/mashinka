use crate::grow::draft_post::DraftPost;
use crate::grow::lang::Lang;
use crate::grow::{KEYWORDS_DELIMITER, KEY_VALUE_DELIMITER, LF, META_DELIMITER};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fmt, fs};

// todo use this.error
#[derive(Debug)]
struct GrowDeserializerError {
    message: String,
}

impl GrowDeserializerError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for GrowDeserializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GrowDeserializerError: {}", &self.message)
    }
}

impl Error for GrowDeserializerError {}

pub fn from_grow_draft_file(draft_path: &PathBuf) -> Result<DraftPost, Box<dyn Error>> {
    let draft_content = fs::read_to_string(draft_path)?;

    from_grow_draft_string(&draft_content)
}

pub fn from_grow_draft_string(draft_string: &str) -> Result<DraftPost, Box<dyn Error>> {
    let mut draft_post = DraftPost::new();
    for (key, value) in deserialize(draft_string) {
        match key {
            "title" => draft_post.title(value),
            "description" => draft_post.description(value),
            "keywords" => draft_post.keywords_as_str(value, KEYWORDS_DELIMITER),
            "lang" => draft_post.lang(Lang::from_str(value)?),
            "content" => draft_post.content(value),
            key => {
                return Err(Box::from(GrowDeserializerError::new(
                    format! {"Unknown key {}", key},
                )))
            }
        };
    }

    Ok(draft_post)
}

/// Преобразует строку в `DraftPost`. Строка должна удовлетворять формату grow записи. Например
/// key: value
///---
/// content
fn deserialize(draft_content: &str) -> HashMap<&str, &str> {
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
