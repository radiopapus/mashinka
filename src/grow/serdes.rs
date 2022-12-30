#![allow(clippy::must_use_candidate)]
use crate::command::Error;
use crate::grow::post::Post;
use std::collections::HashMap;

use crate::grow::{
    ISO8601_DATE_TIME_FORMAT, KEYWORDS_DELIMITER, KEY_VALUE_DELIMITER, LF, META_DELIMITER,
};

/// Преобразует строку в `DraftPost`. Строка должна удовлетворять формату grow записи. Например:
/// key: value
///---
/// content
///
/// # Errors
///
/// Вернет Error при десериализации данных. Meta данные должны быть разделены `META_DELIMITER`, а meta
/// ключ-значение разделены `KEY_VALUE_DELIMITER`.
pub fn deserialize(draft_content: &str) -> Result<HashMap<&str, &str>, Error> {
    let (meta, content) = draft_content
        .trim()
        .split_once(META_DELIMITER)
        .ok_or_else(|| {
            Error::IncorrectFormat(format!(
                "Meta and content should be delimited by {}",
                META_DELIMITER
            ))
        })?;

    let meta_lines: Vec<&str> = meta.trim().split(LF).collect();

    let mut hashmap = HashMap::new();

    for line in meta_lines {
        let (key, value) = line.split_once(KEY_VALUE_DELIMITER).ok_or_else(|| {
            Error::IncorrectFormat(format!(
                "Check meta key value delimiter should be {}",
                KEY_VALUE_DELIMITER
            ))
        })?;
        hashmap.insert(key, value);
    }

    hashmap.insert("content", content);

    Ok(hashmap)
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

pub fn process_template<S: ::std::hash::BuildHasher>(
    mut content: String,
    hashmap: HashMap<&str, String, S>,
) -> String {
    for (key, value) in hashmap {
        content = content.replace(&format!("[{key}]"), &value);
    }
    content
}

#[cfg(test)]
mod tests {
    use crate::command::Error;
    use crate::grow::serdes::deserialize;
    use crate::grow::{KEY_VALUE_DELIMITER, META_DELIMITER};

    #[test]
    fn fail_deserialize_when_meta_content_have_incorrect_delimiter() {
        let err = Error::IncorrectFormat(format!(
            "Meta and content should be delimited by {}",
            META_DELIMITER
        ));

        assert_eq!(err, deserialize("Incorrect value").err().unwrap());
    }

    #[test]
    fn fail_deserialize_when_meta_content_have_incorrect_key_value_delimiter() {
        let err = Error::IncorrectFormat(format!(
            "Check meta key value delimiter should be {}",
            KEY_VALUE_DELIMITER
        ));

        assert_eq!(
            err,
            deserialize("incorrect_meta_value---content").err().unwrap()
        );
    }
}
