use crate::grow::{KEY_VALUE_DELIMITER, LF, META_DELIMITER};
use std::collections::HashMap;

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
