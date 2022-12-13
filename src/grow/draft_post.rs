use chrono::Utc;
use slug::slugify;
use crate::grow::{KEY_VALUE_DELIMITER, KEYWORDS_DELIMITER, LF, MAX_CHARS_IN_DESCRIPTION, MAX_CHARS_IN_TITLE, META_DELIMITER};
use crate::grow::lang::Lang;
use crate::grow::post::Post;

/// Структура для черновика записи. В дальнейшем черновик может быть опубликован (превращен в Post)
#[derive(Debug, PartialEq, Eq)]
pub struct DraftPost {
    /// Заголовок на языке текста lang.
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
    pub fn new() -> DraftPost {
        DraftPost {
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
        assert!(title.chars().count() <= MAX_CHARS_IN_TITLE, "title should be less than {} characters", MAX_CHARS_IN_TITLE);

        self.title = title.to_string();
        self
    }

    /// Задает, очищает от пробелов и проверяет корректность описания записи.
    pub fn description(&mut self, description: &str) -> &mut DraftPost {
        let description = description.trim();

        assert!(!description.is_empty(), "description should not be empty");
        assert!(description.chars().count() <= MAX_CHARS_IN_DESCRIPTION, "description should be less than {} characters", MAX_CHARS_IN_DESCRIPTION);

        self.description = description.to_string();
        self
    }

    /// Задает, очищает от пробелов и проверяет корректность ключевые слова записи.
    pub fn keywords(&mut self, keywords: Vec<String>) -> &mut DraftPost {

        assert!(!keywords.is_empty(), "keywords should not be empty");
        assert!(keywords.len() <= 10, "keywords should be less than 10 keywords");

        self.keywords = keywords.into_iter()
            .map(|keyword| keyword.trim().to_string())
            .collect();
        self
    }

    pub fn keywords_as_str(&mut self, keywords: &str, delimiter: char) -> &mut DraftPost {
        let keywords: Vec<String> = keywords.split(delimiter)
            .map(ToString::to_string)
            .collect();
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

    /// Преобразует DraftPost в Post.
    pub fn to_post(&self) -> Post {
        // The returned "slug" will consist of a-z, 0-9, and '-'. Furthermore, a slug will
        // never contain more than one '-' in a row and will never start or end with '-'.
        // see slugify implementation for details.
        let slug = slugify(&self.title);

        Post {
            title: slug.to_owned(),
            author: "Андрей Добсон".to_string(), // todo заменить на корректное
            published_date_time: Utc::now(),
            slug: slug.to_owned(),
            description: self.description.to_string(),
            keywords: self.keywords.to_owned(),
            lang: self.lang,
            content: self.content.to_string(),
        }
    }

    /// Преобразует строку в DraftPost. Строка должна удовлетворять формату grow записи. Например
    /// key: value
    ///---
    /// content
    pub fn from_string_to_draft_post(draft_data: String) -> DraftPost {
        let (meta, content) = draft_data.trim().split_once(META_DELIMITER)
            .unwrap_or_else(|| panic!("Meta and content should be delimited by {}", META_DELIMITER));

        let meta_lines: Vec<&str> = meta.trim().split(LF)
            .collect();

        let meta_key_values = meta_lines
            .iter()
            .map(|line|
                line.split_once(KEY_VALUE_DELIMITER)
                    .expect("Check meta key value delimiter")
            );

        let mut draft_post = DraftPost::new();

        for (key, value) in meta_key_values {
            match key {
                "title" => draft_post.title(value),
                "description" => draft_post.description(value),
                "keywords" => draft_post.keywords_as_str(value, KEYWORDS_DELIMITER),
                "lang" => draft_post.lang(Lang::from_str(value)),
                _ => panic!("Unknown key")
            };
        }

        draft_post.content(content);

        draft_post
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::grow::draft_post::DraftPost;
    use crate::grow::lang::Lang;
    use crate::grow::{ISO8601_DATE_FORMAT, ISO8601_DATE_TIME_FORMAT, KEY_VALUE_DELIMITER, KEYWORDS_DELIMITER, lang, MAX_CHARS_IN_DESCRIPTION, MAX_CHARS_IN_TITLE};


    const TEST_DRAFT_TITLE: &str = "Перевод - Почему бумага формата А4 имеет размер 297 мм на 210 мм?";

    const TEST_DESCRIPTION: &str = "Тестовое описание для записи";
    const TEST_SLUG: &str = "perevod-pochemu-bumaga-formata-a4-imeet-razmer-297-mm-na-210-mm";

    // Заголовок записи это идентификатор для переводчика, он не равен TEST_DRAFT_TITLE, но равен slug
    const TEST_POST_TITLE: &str = TEST_SLUG;

    const TEST_AUTHOR: &str = "Андрей Добсон";
    const TEST_KEYWORDS_AS_STRING: &str = "бумага,А4,297 мм";
    const TEST_LANG_AS_STRING: &str = "ru";
    const TEST_CONTENT: &str = "_Вкратце: размер листа А0 равен 1 189 мм на 841 мм (1 м<sup>2</sup>). Площадь 1 м<sup>2</sup>
скорее всего выбрана из-за удобства измерения и расчетов. Сотношение сторон примерно равно sqrt2 (1.41) и
выбрано не случайно. Это дает возможность получать листы меньшего размера, сохраняя соотношение сторон.
Таким образом, <i>чтобы получить из листа формата</i> А0 лист формата А4 нужно свернуть лист 4 раза.
 Вот и получается 1 189 / 4 = 297.25, что примерно равно 297 мм._";

    #[derive(Default)]
    struct TestDraftPost {
        title: Option<String>,
        description: Option<String>,
        keywords_as_string: Option<String>,
        content: Option<String>,
    }

    fn generate_test_draft_string(test_draft_post: TestDraftPost) -> String {
        format!(r#"title: {title}
lang: ru
description: {description}
keywords: {keywords_as_string}
---
{content}"#,
                title = test_draft_post.title.unwrap_or(TEST_DRAFT_TITLE.to_string()),
                description = test_draft_post.description.unwrap_or(TEST_DESCRIPTION.to_string()),
                keywords_as_string = test_draft_post.keywords_as_string.unwrap_or(TEST_KEYWORDS_AS_STRING.to_string()),
                content = test_draft_post.content.unwrap_or(TEST_CONTENT.to_string())
        )
    }

    #[test]
    fn test_draft_from_string_conversion_with_default_values() {
        let draft_string = generate_test_draft_string(TestDraftPost::default());

        let draft_post = DraftPost::from_string_to_draft_post(draft_string);

        assert_eq!(TEST_DRAFT_TITLE, draft_post.title);
        assert_eq!(TEST_CONTENT, draft_post.content);
        assert_eq!(TEST_DESCRIPTION, draft_post.description);

        let keywords: Vec<String> = TEST_KEYWORDS_AS_STRING.split(KEYWORDS_DELIMITER)
            .map(ToString::to_string)
            .collect();
        assert_eq!(keywords, draft_post.keywords);
        assert_eq!(Lang::Ru, draft_post.lang);
    }

    #[test]
    #[should_panic(expected = "title should be less than 75 characters")]
    fn fail_draft_from_string_conversion_when_title_out_of_limit() {
        let many_chars_in_title = String::from_utf8(vec![b'X'; MAX_CHARS_IN_TITLE + 1]);
        let test_draft_post = TestDraftPost{
            title: Option::from(many_chars_in_title.unwrap()),
            ..TestDraftPost::default()
        };
        let draft_string = generate_test_draft_string(test_draft_post);
        DraftPost::from_string_to_draft_post(draft_string);
    }

    #[test]
    #[should_panic(expected = "description should be less than 255 characters")]
    fn fail_draft_from_string_conversion_when_description_out_of_limit() {
        let many_chars_in_description = String::from_utf8(vec![b'X'; MAX_CHARS_IN_DESCRIPTION + 1]);
        let test_draft_post = TestDraftPost{
            description: Option::from(many_chars_in_description.unwrap()),
            ..TestDraftPost::default()
        };
        let draft_string = generate_test_draft_string(test_draft_post);
        DraftPost::from_string_to_draft_post(draft_string);
    }

    #[test]
    fn test_draft_to_post_conversion() {
        let draft_string = generate_test_draft_string(TestDraftPost::default());
        let draft_post = DraftPost::from_string_to_draft_post(draft_string);
        let post = draft_post.to_post();

        assert_eq!(TEST_SLUG, post.slug);
        assert_eq!(TEST_DESCRIPTION, post.description);
        assert_eq!(TEST_KEYWORDS_AS_STRING, post.keywords.join(&KEYWORDS_DELIMITER.to_string()));
        assert_eq!(TEST_CONTENT, post.content);
        assert_eq!(TEST_DESCRIPTION, post.description);
        assert_eq!(Lang::Ru, post.lang);
        assert_eq!(TEST_LANG_AS_STRING, post.lang.to_str());
        assert_eq!(TEST_AUTHOR, post.author);
        assert_eq!(TEST_POST_TITLE, post.title);

        let expected_formatted_date_time = Utc::now()
            .format(ISO8601_DATE_FORMAT)
            .to_string();

        let post_formatted_date_time = post.published_date_time
            .format(ISO8601_DATE_FORMAT)
            .to_string();

        assert_eq!(expected_formatted_date_time, post_formatted_date_time);
    }
}