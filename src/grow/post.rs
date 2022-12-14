use chrono::{DateTime, Utc};
use crate::grow::{ISO8601_DATE_FORMAT, KEYWORDS_DELIMITER};
use crate::grow::lang::Lang;

// todo move to the template and read from file
const GROW_POST_PATTERN: &str = r#"
---
$title@: {slug}
author@: {author}
description: {description}
keywords: {keywords}
image: {image}
slug{lang}: {slug}
$dates:
  published: {publish_date}
---
{content}
"#;

#[derive(Debug, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub lang: Lang,
    pub published_date_time: DateTime<Utc>,
    pub slug: String,
    pub content: String,
}

impl Post {
    // Преобразует Post в форматированную строку для grow записи.
    // title здесь это идентификатор - slug, который будет использован для системы перевода
    pub fn to_grow_post(&self) -> String {
        let published_date = self.published_date_time
            .format(ISO8601_DATE_FORMAT)
            .to_string();

        GROW_POST_PATTERN
            .replace("{title}", &self.slug)
            .replace("{author}", &self.author)
            .replace("{description}", &self.description)
            .replace("{keywords}", &self.keywords.join(&KEYWORDS_DELIMITER.to_string()))
            .replace("{image}", "/static/images/default.png")
            .replace("{lang}", self.lang.to_str())
            .replace("{slug}", &self.slug)
            .replace("{publish_date}", &published_date)
            .replace("{content}", &self.content)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use crate::grow::{ISO8601_DATE_FORMAT, DEFAULT_AUTHOR, TEST_CONTENT, TEST_DESCRIPTION, TEST_DRAFT_TITLE, TEST_SLUG};
    use crate::grow::lang::Lang;
    use crate::grow::post::{Post};

    fn generate_fake_post(predefined_date_time: &DateTime<Utc>) -> Post {
        Post {
            title: String::from(TEST_DRAFT_TITLE),
            author: String::from(DEFAULT_AUTHOR),
            description: String::from(TEST_DESCRIPTION),
            lang: Lang::Ru,
            slug: String::from(TEST_SLUG),
            keywords: vec!["бумага".to_string(), "А4".to_string(), "297 мм".to_string()],
            published_date_time: *predefined_date_time,
            content: String::from(TEST_CONTENT),
        }
    }

    #[test]
    fn test_post_to_string() {
        let predefined_date_time = Utc::now();
        let post = generate_fake_post(&predefined_date_time);

        let grow_post_as_string = post.to_grow_post();
        assert!(!grow_post_as_string.is_empty());

        dbg!(&grow_post_as_string);

        // title это slug идентификатор для системы перевода. title из post будет использован в переводе
        let expected_title_is_slug = format!("${}@: {}", "title", &post.slug);
        assert!(grow_post_as_string.contains(&expected_title_is_slug), "exp = {}", expected_title_is_slug);

        let expected_author = format!("{}@: {}", "author", &post.author);
        assert!(grow_post_as_string.contains(&expected_author), "exp = {}", expected_author);

        let expected_description = format!("{}: {}", "description", &post.description);
        assert!(grow_post_as_string.contains(&expected_description), "exp = {}", expected_description);

        let expected_slug = format!("slug{}: {}", &post.lang, &post.slug);
        assert!(grow_post_as_string.contains(&expected_slug), "exp = {}", expected_slug);

        let formatted_naive_date_time = predefined_date_time
            .format(ISO8601_DATE_FORMAT)
            .to_string();

        let expected_publish_date = format!("$dates:\n  published: {}", &formatted_naive_date_time);
        assert!(grow_post_as_string.contains(&expected_publish_date), "exp = {}", expected_publish_date);

        let expected_content = format!("---\n{}", &post.content);
        assert!(grow_post_as_string.contains(&expected_content), "exp = {}", expected_content);
    }
}