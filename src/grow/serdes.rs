#![allow(clippy::must_use_candidate)]
#![allow(clippy::or_fun_call)]

use crate::grow::post::{DraftPost, GrowPost, PostContent};
use std::collections::HashMap;
use std::str::FromStr;
use crate::command::Error;
use crate::grow::builder::{BasePostBuilder};
use crate::grow::lang::{Lang};
use crate::grow::{KEY_VALUE_DELIMITER, KEYWORDS_DELIMITER, LF, META_DELIMITER, TEXT_FIELD_NAME, TITLE_FIELD_NAME,
    DESCRIPTION_FIELD_NAME, KEYWORDS_FIELD_NAME, LANGUAGE_FIELD_NAME, AUTHOR_FIELD_NAME, SLUG_FIELD_NAME_RU,
    SLUG_FIELD_NAME_EN, PUBLISHED_DATE_FIELD_NAME, IMAGE_FIELD_NAME};

pub trait GrowDeserializer<T> {
    fn deserialize(source: &str) -> Result<T, Error>;
}

type GrowMeta = String;
type GrowText = String;

type ParameterName = String;
type ParameterValue = String;

fn split_meta_and_text(value: &str, delimiter: &str) -> Result<(GrowMeta, GrowText), Error> {
    let split_values: Vec<&str> = value.trim()
        .splitn(3, delimiter).collect();

    if split_values.len() != 3 {
        return Err(
            Error::IncorrectFormat(format!("Check post format. Should be {delimiter}\nmeta\n{delimiter}text , content `{}`", delimiter))
        )
    }

    Ok((split_values[1].to_string(), split_values[2].to_string()))
}

fn split_key_value(content: &str, delimiter: &str) -> Result<(ParameterName, ParameterValue), Error> {
    let (key, value) = content.split_once(delimiter).ok_or(
        Error::IncorrectFormat(format!("Meta key value should be delimited by `{}`, content `{}`", delimiter, content))
    )?;
    let key = key.replace(|c: char| !c.is_alphanumeric(), "");
    Ok((key, value.trim().to_string()))
}

fn parse_meta_into_map(content: String) -> Result<HashMap<String, String>, Error> {
    let meta_lines: Vec<String> = content.trim().split(LF).map(ToString::to_string).collect(); // skip first META_DELIMITER delimiter

    let mut hashmap: HashMap<String, String> = HashMap::new();

    for line in meta_lines {
        let (parameter_name, parameter_value) = split_key_value(&line, KEY_VALUE_DELIMITER)?;

        // пропускаем варианты вроде dates: и другие некорректные варианты. Интересует только key:value формат.
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
        for (parameter_name, value) in map {
            match parameter_name.as_str() {
                TITLE_FIELD_NAME => builder.title(value)?,
                DESCRIPTION_FIELD_NAME => builder.description(value)?,
                KEYWORDS_FIELD_NAME => builder.keywords_as_str(value, KEYWORDS_DELIMITER)?,
                LANGUAGE_FIELD_NAME => builder.lang(Lang::from_str(value.as_str()).map_err(Error::UnknownLang)?)?,
                TEXT_FIELD_NAME => builder.text(value)?,
                unknown => return Err(Error::UnknownKey(unknown.to_string())),
            };
        }

        Ok(builder.build())
    }
}

impl GrowDeserializer<GrowPost> for GrowPost {
    fn deserialize(source: &str) -> Result<GrowPost, Error> {
        let map = convert_grow_content_to_hashmap(source)?;
        let mut builder = GrowPost::builder();

        for (parameter_name, parameter_value) in map {
            match parameter_name.as_str() {
                AUTHOR_FIELD_NAME => builder.author(parameter_value)?,
                SLUG_FIELD_NAME_RU | SLUG_FIELD_NAME_EN => builder.slug(parameter_value)?,
                PUBLISHED_DATE_FIELD_NAME => builder.published_at_str(parameter_value)?,
                IMAGE_FIELD_NAME => builder.image(parameter_value)?,
                // todo fetch title from translation. grow record does not contain title actually or investigate?
                TITLE_FIELD_NAME => builder.title(parameter_value.clone())?,
                DESCRIPTION_FIELD_NAME => builder.description(parameter_value)?,
                KEYWORDS_FIELD_NAME => builder.keywords_as_str(parameter_value, KEYWORDS_DELIMITER)?,
                TEXT_FIELD_NAME => builder.text(parameter_value)?,
                LANGUAGE_FIELD_NAME => builder.lang(Lang::from_str(&parameter_value).map_err(Error::UnknownLang)?)?,
                key => return Err(Error::UnknownKey(key.to_string())),
            };
        }

        Ok(builder.build())
    }
}

pub fn process_template<S: std::hash::BuildHasher>(
    mut template: String,
    hashmap: HashMap<&str, String, S>,
) -> String {
    for (key, value) in hashmap {
        template = template.replace(&format!("[{key}]"), &value);
    }
    template
}

#[cfg(test)]
mod tests {
    use crate::grow::{KEY_VALUE_DELIMITER};
    use std::{assert_eq, format};
    use crate::command::Error;
    use crate::grow::post::{DraftPost};
    use crate::grow::serdes::{GrowDeserializer};

    #[test]
    fn fail_deserialize_when_meta_content_have_incorrect_delimiter() {
        let err = Error::IncorrectFormat("Check post format. Should be ---\nmeta\n---text , content `---`".to_string());

        assert_eq!(err, DraftPost::deserialize("Incorrect value").err().unwrap());
    }

    #[test]
    fn fail_deserialize_when_meta_content_have_incorrect_key_value_delimiter() {
        let err = Error::IncorrectFormat(format!("Meta key value should be delimited by `{}`, content `incorrect_meta_value`", KEY_VALUE_DELIMITER));

        assert_eq!(err, DraftPost::deserialize("---\nincorrect_meta_value\n---content").err().unwrap());
    }
}
