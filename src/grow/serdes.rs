#![allow(clippy::must_use_candidate)]
#![allow(clippy::or_fun_call)]

use crate::grow::post::{DraftPost, GrowPost, PostContent};
use crate::grow::{KEY_VALUE_DELIMITER, LF, META_DELIMITER, TEXT_FIELD_NAME};
use std::collections::HashMap;
use crate::command::Error;
use crate::grow::builder::{BasePostBuilder};

pub trait GrowDeserializer<T> {
    fn deserialize(source: &str) -> Result<T, Error>;
}

type ParameterName = String;
type ParameterValue = String;

fn split_meta_and_text(value: &str, delimiter: &str) -> Result<(String, String), Error> {
    let (meta, text) = value
        .trim()
        .rsplit_once(delimiter)
        .ok_or(Error::IncorrectFormat(format!("Meta and text should be delimited by `{}`",delimiter)))?;

    Ok((meta.to_string(), text.to_string()))
}

fn split_key_value(content: &str, delimiter: &str) -> Result<(ParameterName, ParameterValue), Error> {
    let (key, value) = content.split_once(delimiter).ok_or(
        Error::IncorrectFormat(format!("Meta key value should be delimited by `{}`", delimiter))
    )?;
    let key = key.replace(|c: char| !c.is_alphanumeric(), "");
    Ok((key, value.trim().to_string()))
}

fn parse_meta_into_map(content: String) -> Result<HashMap<String, String>, Error> {
    let meta_lines: Vec<String> = content.trim().split(LF).skip(1).map(ToString::to_string).collect(); // skip first META_DELIMITER delimiter

    let mut hashmap: HashMap<String, String> = HashMap::new();

    for line in meta_lines {
        let (parameter_name, parameter_value) = split_key_value(&line, KEY_VALUE_DELIMITER)?;

        if parameter_name.is_empty() || parameter_value.is_empty() {
            continue
        }

        hashmap.insert(parameter_name.to_string(), parameter_value.to_string());
    }

    Ok(hashmap)
}

fn convert_grow_content_to_hashmap(content: &str) -> Result<HashMap<String, String>, Error> {
    let (meta, text) = split_meta_and_text(content, META_DELIMITER)?;
    let mut map = parse_meta_into_map(meta)?;
    map.insert(TEXT_FIELD_NAME.to_string(), text);

    Ok(map)
}

impl GrowDeserializer<DraftPost> for DraftPost {
    fn deserialize(source: &str) -> Result<DraftPost, Error> {
        let map = convert_grow_content_to_hashmap(source)?;
        let mut builder = DraftPost::builder();

        builder.build_from_hashmap(map)
    }
}

impl GrowDeserializer<GrowPost> for GrowPost {
    fn deserialize(source: &str) -> Result<GrowPost, Error> {
        let map = convert_grow_content_to_hashmap(source)?;
        let mut builder = GrowPost::builder();

        builder.build_from_hashmap(map)
    }
}

pub fn process_template<S: std::hash::BuildHasher>(
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
    use crate::grow::{KEY_VALUE_DELIMITER, META_DELIMITER};
    use std::{assert_eq, format};
    use crate::command::Error;
    use crate::grow::post::DraftPost;
    use crate::grow::serdes::{GrowDeserializer};

    #[test]
    fn fail_deserialize_when_meta_content_have_incorrect_delimiter() {
        let err = Error::IncorrectFormat(format!("Meta and text should be delimited by `{}`", META_DELIMITER));

        assert_eq!(err, DraftPost::deserialize("Incorrect value").err().unwrap());
    }

    #[test]
    fn fail_deserialize_when_meta_content_have_incorrect_key_value_delimiter() {
        let err = Error::IncorrectFormat(format!("Meta key value should be delimited by `{}`", KEY_VALUE_DELIMITER));

        assert_eq!(err, DraftPost::deserialize("---\nincorrect_meta_value\n---content").err().unwrap());
    }
}
