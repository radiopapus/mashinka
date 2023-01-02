#![allow(clippy::must_use_candidate)]

use crate::command::Error;
use crate::grow::lang::Lang;
use crate::grow::post::Post;
use crate::grow::serdes::deserialize_to_draft_post;
use crate::grow::{
    DEFAULT_AUTHOR, DEFAULT_AUTHOR_EN, MAX_CHARS_IN_DESCRIPTION, MAX_CHARS_IN_TITLE,
    MAX_KEYWORDS_COUNT,
};
use chrono::Utc;
use slug::slugify;
use std::fs;
use std::path::PathBuf;

/// Структура для черновика записи. В дальнейшем черновик может быть опубликован (превращен в Post)
#[derive(Debug, PartialEq, Eq)]
pub struct DraftPost {
    /// Заголовок на языке текста.
    title: String,
    /// Описание записи
    description: String,
    /// Ключевые слова записи
    keywords: Vec<String>,
    /// Язык записи
    lang: Lang,
    /// Текст записи
    content: String,
}

impl Default for DraftPost {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            keywords: vec![],
            lang: Lang::Ru,
            content: String::new(),
        }
    }
}

impl DraftPost {
    /// Создает пустой черновик.
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    /// Возвращает `DraftPost` на основе данных файла `draft_path`
    ///
    /// # Errors
    ///
    /// Вернет `Error` если файл не может быть прочитан.
    pub fn from_grow_draft_file(draft_path: &PathBuf) -> Result<DraftPost, Error> {
        let draft_content = fs::read_to_string(draft_path).map_err(Error::ReadDraft)?;

        deserialize_to_draft_post(&draft_content)
    }

    /// Задает, очищает от пробелов и проверяет корректность заголовка записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если `title` пустой или более `MAX_CHARS_IN_TITLE` символов.
    pub fn title(&mut self, title: &str) -> Result<&mut DraftPost, Error> {
        let title = title.trim();

        if title.is_empty() {
            return Err(Error::EmptyValue(String::from("title")));
        }

        if title.chars().count() >= MAX_CHARS_IN_TITLE {
            return Err(Error::ValueTooLong(
                String::from("title"),
                MAX_CHARS_IN_TITLE,
            ));
        }

        self.title = title.to_string();
        Ok(self)
    }

    /// Задает, очищает от пробелов и проверяет корректность описания записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если `description` пустой или более `MAX_CHARS_IN_DESCRIPTION` символов.
    pub fn description(&mut self, description: &str) -> Result<&mut DraftPost, Error> {
        let description = description.trim();

        if description.is_empty() {
            return Err(Error::EmptyValue(String::from("description")));
        }

        if description.chars().count() >= MAX_CHARS_IN_DESCRIPTION {
            return Err(Error::ValueTooLong(
                String::from("description"),
                MAX_CHARS_IN_DESCRIPTION,
            ));
        }

        self.description = description.to_string();
        Ok(self)
    }

    /// Задает, очищает от пробелов и проверяет корректность ключевых слов записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если keywords пустой или более `MAX_KEYWORDS_COUNT` элементов.
    pub fn keywords(&mut self, keywords: Vec<String>) -> Result<&mut DraftPost, Error> {
        if keywords.is_empty() {
            return Err(Error::EmptyValue(String::from("keywords")));
        }

        if keywords.len() >= MAX_KEYWORDS_COUNT {
            return Err(Error::ValueTooLong(
                String::from("keywords"),
                MAX_KEYWORDS_COUNT,
            ));
        }

        for k in keywords {
            self.keywords.push(k.trim().to_string());
        }

        Ok(self)
    }

    /// Аналогично keywords, но в качестве параметров можно передать строку и указать разделитель.
    /// # Errors
    ///
    /// См. keywords
    pub fn keywords_as_str(
        &mut self,
        keywords: &str,
        delimiter: &str,
    ) -> Result<&mut DraftPost, Error> {
        let keywords: Vec<String> = keywords.split(delimiter).map(ToString::to_string).collect();
        self.keywords(keywords)?;
        Ok(self)
    }

    /// Задает язык записи.
    pub fn lang(&mut self, lang: Lang) -> &mut DraftPost {
        self.lang = lang;
        self
    }

    /// Задает текст записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если content пустой.
    pub fn content(&mut self, content: &str) -> Result<&mut DraftPost, Error> {
        let content = content.trim();

        if content.is_empty() {
            return Err(Error::EmptyValue(String::from("content")));
        }

        self.content = content.to_string();
        Ok(self)
    }

    /// Помечает черновик как готовый для публикации.
    /// Технически преобразует структуру `DraftPost` в `Post`.
    pub fn approve(&self) -> Post {
        // Результат "slug" состоит из символов a-z, 0-9 и '-'.
        // Никогда не содержит более одного '-' и не начинается с '-'.
        // see slugify implementation for details.
        let slug = slugify(&self.title);

        let mut author = DEFAULT_AUTHOR.to_string();

        if self.lang != Lang::Ru {
            author = DEFAULT_AUTHOR_EN.to_string();
        }

        Post {
            title: self.title.clone(),
            author,
            published_date_time: Utc::now(),
            slug,
            description: self.description.clone(),
            keywords: self.keywords.clone(),
            lang: self.lang,
            draft_content: self.content.clone(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::or_fun_call)]
mod tests {
    use crate::grow::lang::Lang;
    const TEST_POST_TITLE: &str = "Это тестовый заголовок";
    const TEST_KEYWORDS_AS_STRING: &str = "бумага,А4,297 мм";
    pub const TEST_LANG_AS_STRING: &str = "ru";

    pub const TEST_DRAFT_TEMPLATE: &str = r#"
title: [title]
lang: [lang]
description: [description]
keywords: [keywords]
---

[content]
"#;

    use crate::grow::{
        DEFAULT_AUTHOR, ISO8601_DATE_FORMAT, KEYWORDS_DELIMITER, MAX_CHARS_IN_DESCRIPTION,
        MAX_CHARS_IN_TITLE, TEST_CONTENT, TEST_DESCRIPTION, TEST_DRAFT_TITLE, TEST_SLUG,
    };

    use crate::command::Error::ValueTooLong;
    use crate::grow::serdes::{deserialize_to_draft_post, process_template};
    use chrono::Utc;
    use std::collections::HashMap;

    #[derive(Default)]
    struct TestDraftPost {
        title: Option<String>,
        description: Option<String>,
        keywords_as_string: Option<String>,
        content: Option<String>,
    }

    fn generate_test_draft_string(test_draft_post: TestDraftPost) -> String {
        let title = test_draft_post
            .title
            .unwrap_or(TEST_DRAFT_TITLE.to_string());

        let description = test_draft_post
            .description
            .unwrap_or(TEST_DESCRIPTION.to_string());

        let keywords_as_string = test_draft_post
            .keywords_as_string
            .unwrap_or(TEST_KEYWORDS_AS_STRING.to_string());

        let content = test_draft_post.content.unwrap_or(TEST_CONTENT.to_string());

        let hashmap_with_defaults = HashMap::from(
            [
                ("title", title),
                ("description", description),
                ("keywords", keywords_as_string),
                ("content", content),
                ("lang", Lang::Ru.to_string()),
            ]
            .map(|(k, v)| (k, v)),
        );

        process_template(TEST_DRAFT_TEMPLATE.to_owned(), hashmap_with_defaults)
    }

    #[test]
    fn test_draft_from_string_conversion_with_default_values() {
        let draft_content = generate_test_draft_string(TestDraftPost::default());

        let draft_post = deserialize_to_draft_post(&draft_content).unwrap();

        assert_eq!(TEST_DRAFT_TITLE, draft_post.title);
        assert_eq!(TEST_CONTENT, draft_post.content);
        assert_eq!(TEST_DESCRIPTION, draft_post.description);

        let keywords: Vec<String> = TEST_KEYWORDS_AS_STRING
            .split(KEYWORDS_DELIMITER)
            .map(ToString::to_string)
            .collect();
        assert_eq!(keywords, draft_post.keywords);
        assert_eq!(Lang::Ru, draft_post.lang);
    }

    #[test]
    fn fail_draft_from_string_conversion_when_title_out_of_limit() {
        let many_chars_in_title = String::from_utf8(vec![b'X'; MAX_CHARS_IN_TITLE + 1]);
        let test_draft_post = TestDraftPost {
            title: Option::from(many_chars_in_title.unwrap()),
            ..TestDraftPost::default()
        };
        let draft_content = generate_test_draft_string(test_draft_post);
        let result = deserialize_to_draft_post(&draft_content);
        assert_eq!(
            ValueTooLong("title".to_string(), MAX_CHARS_IN_TITLE),
            result.err().unwrap()
        )
    }

    #[test]
    fn fail_draft_from_string_conversion_when_description_out_of_limit() {
        let many_chars_in_description = String::from_utf8(vec![b'X'; MAX_CHARS_IN_DESCRIPTION + 1]);
        let test_draft_post = TestDraftPost {
            description: Option::from(many_chars_in_description.unwrap()),
            ..TestDraftPost::default()
        };
        let draft_content = generate_test_draft_string(test_draft_post);
        let result = deserialize_to_draft_post(&draft_content);
        assert_eq!(
            ValueTooLong("description".to_string(), MAX_CHARS_IN_DESCRIPTION),
            result.err().unwrap()
        )
    }

    #[test]
    fn test_draft_to_post_conversion() {
        let draft_string = generate_test_draft_string(TestDraftPost::default());
        let draft_post = deserialize_to_draft_post(&draft_string).unwrap();
        let post = draft_post.approve();

        assert_eq!(TEST_SLUG, post.slug);
        assert_eq!(TEST_DESCRIPTION, post.description);
        assert_eq!(
            TEST_KEYWORDS_AS_STRING,
            post.keywords.join(KEYWORDS_DELIMITER)
        );
        assert_eq!(TEST_CONTENT, post.draft_content);
        assert_eq!(TEST_DESCRIPTION, post.description);
        assert_eq!(Lang::Ru, post.lang);
        assert_eq!(TEST_LANG_AS_STRING, post.lang.to_string().to_lowercase());
        assert_eq!(DEFAULT_AUTHOR, post.author);
        assert_eq!(TEST_POST_TITLE, post.title);

        let expected_formatted_date_time = Utc::now().format(ISO8601_DATE_FORMAT).to_string();

        let post_formatted_date_time = post
            .published_date_time
            .format(ISO8601_DATE_FORMAT)
            .to_string();

        assert_eq!(expected_formatted_date_time, post_formatted_date_time);
    }
}
