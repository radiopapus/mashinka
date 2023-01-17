#![allow(clippy::must_use_candidate)]

use std::collections::HashMap;
use std::fs;
use crate::command::Error;
use crate::grow::lang::Lang;
use crate::grow::serdes::{GrowDeserializer, process_template};
use crate::grow::{AUTHOR_FIELD_NAME, DEFAULT_AUTHOR, DEFAULT_AUTHOR_EN, DESCRIPTION_FIELD_NAME, DRAFT_TEMPLATE, IMAGE_FIELD_NAME, ISO8601_DATE_FORMAT, ISO8601_DATE_TIME_FORMAT, KEYWORDS_DELIMITER, KEYWORDS_FIELD_NAME, LANGUAGE_FIELD_NAME, POST_TEMPLATE, PUBLISHED_DATE_FIELD_NAME, SLUG_FIELD_NAME, TEXT_FIELD_NAME, TITLE_FIELD_NAME, TRANSLATION_TEMPLATE};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use regex::{Regex};
use slug::slugify;
use crate::grow::builder::{BasePostBuilder, DraftPostBuilder, GrowPostBuilder, PostBuilder};

pub trait PostContent<B> {
    fn new() -> Self;
    fn builder() -> B where B: PostBuilder;
}

impl PostContent<DraftPostBuilder> for DraftPost {
    fn new() -> Self {
        DraftPost::default()
    }
    fn builder() -> DraftPostBuilder {
        DraftPostBuilder::new()
    }
}

impl PostContent<GrowPostBuilder> for GrowPost {
    fn new() -> Self {
        GrowPost::default()
    }
    fn builder() -> GrowPostBuilder {
        GrowPostBuilder::new()
    }
}

/// Структура для черновика записи. В дальнейшем черновик может быть опубликован (превращен в Post)
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DraftPost {
    /// Заголовок на языке текста.
    pub title: String,
    /// Описание записи
    pub description: String,
    /// Ключевые слова записи
    pub keywords: Vec<String>,
    /// Язык записи
    pub lang: Lang,
    /// Текст записи
    pub text: String,
}

/// В Rust миллиард всяких трейтов, которые можно реализовать для вашего типа. Здесь преобразуем
/// DraftPost в строку согласно шаблону.
impl ToString for DraftPost {
    fn to_string(&self) -> String {
        process_template(DRAFT_TEMPLATE.to_string(), self.as_hashmap())
    }
}

impl DraftPost {
    fn as_hashmap(&self) -> HashMap<&str, String> {
        HashMap::from([
            (TITLE_FIELD_NAME, self.title.clone()),
            (DESCRIPTION_FIELD_NAME, self.description.clone()),
            (LANGUAGE_FIELD_NAME, self.lang.to_string()),
            (TEXT_FIELD_NAME, self.text.clone()),
            (KEYWORDS_FIELD_NAME, self.keywords.join(KEYWORDS_DELIMITER)),
        ]).into_iter().collect()
    }

    /// Помечает черновик как готовый для публикации.
    /// Технически преобразует структуру `DraftPost` в `ApprovedPost`.
    pub fn approve(&self) -> ApprovedPost {
        // Результат "slug" состоит из символов a-z, 0-9 и '-'.
        // Никогда не содержит более одного '-' и не начинается с '-'.
        // see slugify implementation for details.
        let slug = slugify(&self.title);

        let author = if self.lang != Lang::Ru { DEFAULT_AUTHOR_EN } else { DEFAULT_AUTHOR };

        ApprovedPost {
            title: self.title.clone(),
            author: author.to_string(),
            slug,
            description: self.description.clone(),
            keywords: self.keywords.clone(),
            lang: self.lang,
            text: self.text.clone(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ApprovedPost {
    pub title: String,
    pub author: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub lang: Lang,
    pub slug: String,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GrowPost {
    pub title: String,
    pub author: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub lang: Lang,
    pub published_at: DateTime<Utc>,
    pub slug: String,
    pub text: String,
}

impl GrowPost {
    pub fn get_posts_by_lang(base_posts_path: &Path, lang: Lang) -> Result<Vec<Self>, Error> {
        let posts_path = fs::read_dir(
            &base_posts_path.join(lang.to_lowercase())
        ).map_err(Error::ReadDir)?;

        let mut posts: Vec<Self> = Vec::new();

        for file in posts_path {
            let entry_path = file.map_err(Error::ReadFile)?.path();

            if entry_path.is_dir() { continue } // skip directory

            let file_content = fs::read_to_string(&entry_path).map_err(Error::ReadFile)?;
            //todo detect lang from grow post?
            let mut grow_post = Self::deserialize(&file_content)?;
            grow_post.lang = lang;
            posts.push(grow_post)
        }

        Ok(posts)
    }

    fn as_hashmap(&self) -> HashMap<&str, String> {
        HashMap::from([
            (TITLE_FIELD_NAME, self.title.clone()),
            (AUTHOR_FIELD_NAME, self.author.clone(), ),
            (DESCRIPTION_FIELD_NAME, self.description.clone(), ),
            (IMAGE_FIELD_NAME, "/static/images/default.png".to_string()),
            (LANGUAGE_FIELD_NAME, self.lang.to_string()),
            (SLUG_FIELD_NAME, self.slug.clone()),
            (TEXT_FIELD_NAME, self.text.clone()),
            (PUBLISHED_DATE_FIELD_NAME, self.published_at.format(ISO8601_DATE_TIME_FORMAT).to_string()),
            (KEYWORDS_FIELD_NAME, self.keywords.join(KEYWORDS_DELIMITER)),
        ]).into_iter().collect()
    }
}

/// Преобразует строку в `GrowPost`. Строка должна удовлетворять формату grow записи. Например:
/// key: value
///---
/// content
///
/// # Errors
/// Вернет Error при десериализации данных. Meta данные должны быть разделены `META_DELIMITER`, а meta
/// ключ-значение разделены `KEY_VALUE_DELIMITER`.
/// Доступные поля `title`, `description`,`keywords`, `lang`, `content`

impl ToString for GrowPost {
    fn to_string(&self) -> String {
        process_template(POST_TEMPLATE.to_string(), self.as_hashmap())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GrowPostTranslation {
    pub id: String, // slug for us
    pub translated_value: String,
}

impl GrowPostTranslation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_translations(path: &PathBuf) -> Result<Vec<GrowPostTranslation>, Error> {
        let translation_content = fs::read_to_string(path).map_err(Error::ReadFile)?;
        let mut vec = Vec::new();

        let re = Regex::new(r#"msgid "(.*)"\nmsgstr "(.*)""#).unwrap();

        for cap in re.captures_iter(translation_content.as_str()) {
            vec.push(GrowPostTranslation { id: cap[1].to_string(), translated_value: cap[2].to_string() });
        }

        Ok(vec)
    }
}

/// Преобразует GrowPostTranslation в форматированную строку для переводов.
/// title здесь это идентификатор - slug, который будет использован для системы перевода
impl ToString for GrowPostTranslation {
    fn to_string(&self) -> String {
        process_template(TRANSLATION_TEMPLATE.to_string(),
            HashMap::from([("id", self.id.to_string()), ("value", self.translated_value.to_string())]),
        )
    }
}

pub struct WriterWrapper {}

impl WriterWrapper {
    fn _write_to_file(file: &PathBuf, append: bool, file_content: &String) -> Result<(), Error> {
        let mut f = File::options()
            .append(append)
            .create(true)
            .write(true)
            .open(file)
            .map_err(Error::WriteFile)?;
        f.write_all(file_content.as_bytes())
            .map_err(Error::WriteFile)?;
        Ok(())
    }

    pub fn write_file(file: &PathBuf, content: &String) -> Result<(), Error> {
        Self::_write_to_file(file, false, content)
    }

    pub fn write_file_with_append(file: &PathBuf, content: &String) -> Result<(), Error> {
        Self::_write_to_file(file, true, content)
    }
}

impl GrowPost {
    pub fn build_post_path(&self, posts_path: &Path) -> PathBuf {
        let published_at = self.published_at.format(ISO8601_DATE_FORMAT).to_string();
        let slug = self.slug.clone();
        let lang = self.lang.to_lowercase();
        posts_path.join(self.lang.to_lowercase())
            .join(format!("{published_at}-{slug}@{lang}.md"))
    }
}

impl ApprovedPost {
    ///
    /// Публикует `Post` по путям `posts_path` (запись) и `translation_path`(перевод).
    /// # Errors
    ///
    /// Вернет `Error` при записи файлов возникнут проблемы.
    pub fn to_grow_post(&self) -> Result<GrowPost, Error> {
        Ok(GrowPostBuilder::new()
            .title(self.title.clone())?
            .description(self.description.clone())?
            .author(self.author.clone())?
            .keywords(self.keywords.clone())?
            .lang(self.lang)?
            .published_at(Utc::now())?
            .slug(self.slug.clone())?
            .text(self.text.clone())?
            .build()
        )
    }
}


#[cfg(test)]
#[allow(clippy::or_fun_call)]
mod tests {
    use crate::grow::{MAX_CHARS_IN_DESCRIPTION, MAX_CHARS_IN_TITLE};
    use crate::grow::serdes::{GrowDeserializer};
    use crate::command::Error::ValueTooLong;
    use crate::grow::post::{DraftPost, GrowPost};

    #[test]
    fn test_draft_from_string_conversion_with_default_values() {
        let default_draft = DraftPost {
            text: "text".to_string(),
            ..DraftPost::default()
        };

        // let DraftPost {text, title, ..} = default_draft;

        let draft_post = DraftPost::deserialize(&default_draft.to_string()).unwrap();

        assert_eq!(default_draft.title, draft_post.title);
        assert_eq!(default_draft.text, draft_post.text);
        assert_eq!(default_draft.description, draft_post.description);
        assert_eq!(default_draft.keywords, draft_post.keywords);
        assert_eq!(default_draft.lang, draft_post.lang);
    }

    #[test]
    fn fail_draft_from_string_conversion_when_title_out_of_limit() {
        let too_many_chars_in_title = String::from_utf8(vec![b'X'; MAX_CHARS_IN_TITLE + 1]);
        let draft_content = DraftPost {
            title: too_many_chars_in_title.unwrap(),
            text: "test_text".to_string(),
            ..DraftPost::default()
        };

        let result = DraftPost::deserialize(&draft_content.to_string());
        assert_eq!(
            ValueTooLong("title".to_string(), draft_content.title, MAX_CHARS_IN_TITLE),
            result.err().unwrap()
        )
    }

    #[test]
    fn fail_draft_from_string_conversion_when_description_out_of_limit() {
        let too_many_chars_in_description = String::from_utf8(vec![b'X'; MAX_CHARS_IN_DESCRIPTION + 1]);
        let draft_content = DraftPost {
            description: too_many_chars_in_description.unwrap(),
            text: "test_text".to_string(),
            ..DraftPost::default()
        };
        let result = DraftPost::deserialize(&draft_content.to_string());
        assert_eq!(
            ValueTooLong("description".to_string(), draft_content.description, MAX_CHARS_IN_DESCRIPTION),
            result.err().unwrap()
        )
    }

    #[test]
    fn test_draft_to_approved_post_conversion() {
        let default_draft = DraftPost {
            text: "txt".to_string(),
            ..DraftPost::default()
        };
        let draft_post = DraftPost::deserialize(&default_draft.to_string()).unwrap();

        let approved_post = draft_post.approve();

        assert_eq!(draft_post.title, approved_post.title);
        assert_eq!(draft_post.description, approved_post.description);
        assert_eq!(draft_post.keywords, approved_post.keywords);
        assert_eq!(draft_post.text, approved_post.text);
        assert_eq!(draft_post.lang, approved_post.lang);
    }

    #[test]
    fn test_grow_from_string_conversion_with_default_values() {
        let default_grow_post = GrowPost {
            text: "text".to_string(),
            author: "Author".to_string(),
            ..GrowPost::default()
        };

        let grow_post = GrowPost::deserialize(&default_grow_post.to_string()).unwrap();

        assert_eq!(default_grow_post.title, grow_post.title);
        assert_eq!(default_grow_post.text, grow_post.text);
        assert_eq!(default_grow_post.description, grow_post.description);
        assert_eq!(default_grow_post.keywords, grow_post.keywords);
        assert_eq!(default_grow_post.lang, grow_post.lang);
        assert_eq!(default_grow_post.author, grow_post.author);
        assert_eq!(default_grow_post.slug, grow_post.slug);
    }
}
