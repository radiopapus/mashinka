use std::collections::HashMap;

pub mod grow_draft_deserializer;
pub mod grow_post_serializer;

pub fn process_template(mut content: String, hashmap: HashMap<&str, String>) -> String {
    for (key, value) in hashmap {
        content = content.replace(&format!("[{key}]"), &value);
    }
    content
}