use crate::grow::lang::Lang;
use crate::grow::post::Post;
use crate::grow::{
    DEFAULT_AUTHOR, DEFAULT_AUTHOR_EN, MAX_CHARS_IN_DESCRIPTION, MAX_CHARS_IN_TITLE,
};
use chrono::Utc;
use slug::slugify;

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

impl DraftPost {
    /// Создает пустой черновик.
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            keywords: vec![],
            lang: Lang::Ru,
            content: String::new(),
        }
    }

    /// Задает, очищает от пробелов и проверяет корректность заголовка записи.
    pub fn title(&mut self, title: &str) -> &mut DraftPost {
        let title = title.trim();

        assert!(!title.is_empty(), "title should not be empty");
        assert!(
            title.chars().count() <= MAX_CHARS_IN_TITLE,
            "title should be less than {} characters",
            MAX_CHARS_IN_TITLE
        );

        self.title = title.to_string();
        self
    }

    /// Задает, очищает от пробелов и проверяет корректность описания записи.
    pub fn description(&mut self, description: &str) -> &mut DraftPost {
        let description = description.trim();

        assert!(!description.is_empty(), "description should not be empty");
        assert!(
            description.chars().count() <= MAX_CHARS_IN_DESCRIPTION,
            "description should be less than {} characters",
            MAX_CHARS_IN_DESCRIPTION
        );

        self.description = description.to_string();
        self
    }

    /// Задает, очищает от пробелов и проверяет корректность ключевых слов записи.
    pub fn keywords(&mut self, keywords: Vec<String>) -> &mut DraftPost {
        assert!(!keywords.is_empty(), "keywords should not be empty");
        assert!(
            keywords.len() <= 10,
            "keywords should be less than 10 keywords"
        );

        self.keywords = keywords
            .into_iter()
            .map(|keyword| keyword.trim().to_string())
            .collect();
        self
    }

    /// Аналогично keywords, но в качестве параметров можно передать строку и указать разделитель.
    pub fn keywords_as_str(&mut self, keywords: &str, delimiter: &str) -> &mut DraftPost {
        let keywords: Vec<String> = keywords.split(delimiter).map(ToString::to_string).collect();
        let _ = &mut self.keywords(keywords);
        self
    }

    /// Задает язык записи.
    pub fn lang(&mut self, lang: Lang) -> &mut DraftPost {
        self.lang = lang;
        self
    }

    pub fn content(&mut self, content: &str) -> &mut DraftPost {
        let content = content.trim();

        assert!(!content.is_empty(), "content should not be empty");

        self.content = content.to_string();
        self
    }

    /// Помечает черновик как готовый для публикации.
    /// Технически преобразует структуру DraftPost в Post.
    pub fn approve(&self) -> Post {
        // Результат "slug" состоит из символов a-z, 0-9 и '-'.
        // Никогда не содержит более одного '-' и не начинается с '-'.
        // see slugify implementation for details.
        let slug = slugify(&self.title);

        let author = if self.lang == Lang::Ru {
            DEFAULT_AUTHOR.to_string()
        } else {
            DEFAULT_AUTHOR_EN.to_string()
        };

        Post {
            title: self.title.to_owned(),
            author,
            published_date_time: Utc::now(),
            slug: slug.to_owned(),
            description: self.description.to_owned(),
            keywords: self.keywords.to_owned(),
            lang: self.lang,
            draft_content: self.content.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grow::lang::Lang;
    use crate::grow::{
        DEFAULT_AUTHOR, ISO8601_DATE_FORMAT, KEYWORDS_DELIMITER, MAX_CHARS_IN_DESCRIPTION,
        MAX_CHARS_IN_TITLE,
    };

    use crate::grow::draft_post::DraftPost;
    use crate::grow::serdes::grow_draft_deserializer::from_grow_draft_string;
    use crate::grow::serdes::process_template;
    use chrono::Utc;
    use mashinka::{
        TEST_CONTENT, TEST_DESCRIPTION, TEST_DRAFT_TEMPLATE, TEST_DRAFT_TITLE,
        TEST_KEYWORDS_AS_STRING, TEST_LANG_AS_STRING, TEST_POST_TITLE, TEST_SLUG,
    };
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
        let draft_string = generate_test_draft_string(TestDraftPost::default());

        let draft_post = from_grow_draft_string(&draft_string);

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
    #[should_panic(expected = "title should be less than 75 characters")]
    fn fail_draft_from_string_conversion_when_title_out_of_limit() {
        let many_chars_in_title = String::from_utf8(vec![b'X'; MAX_CHARS_IN_TITLE + 1]);
        let test_draft_post = TestDraftPost {
            title: Option::from(many_chars_in_title.unwrap()),
            ..TestDraftPost::default()
        };
        let draft_string = generate_test_draft_string(test_draft_post);
        from_grow_draft_string(&draft_string);
    }

    #[test]
    #[should_panic(expected = "description should be less than 255 characters")]
    fn fail_draft_from_string_conversion_when_description_out_of_limit() {
        let many_chars_in_description = String::from_utf8(vec![b'X'; MAX_CHARS_IN_DESCRIPTION + 1]);
        let test_draft_post = TestDraftPost {
            description: Option::from(many_chars_in_description.unwrap()),
            ..TestDraftPost::default()
        };
        let draft_string = generate_test_draft_string(test_draft_post);
        from_grow_draft_string(&draft_string);
    }

    #[test]
    fn test_draft_to_post_conversion() {
        let draft_string = generate_test_draft_string(TestDraftPost::default());
        let draft_post = from_grow_draft_string(&draft_string);
        let post = draft_post.approve();

        assert_eq!(TEST_SLUG, post.slug);
        assert_eq!(TEST_DESCRIPTION, post.description);
        assert_eq!(
            TEST_KEYWORDS_AS_STRING,
            post.keywords.join(&KEYWORDS_DELIMITER.to_string())
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
