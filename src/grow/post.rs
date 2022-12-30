use crate::grow::lang::Lang;
use crate::grow::serdes::grow_post_serializer::serialize_with_template;
use crate::grow::serdes::process_template;
use crate::grow::{ISO8601_DATE_FORMAT, POST_TEMPLATE, TRANSLATION_TEMPLATE};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

type PostPath = PathBuf;
type TranslationPath = PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub lang: Lang,
    pub published_date_time: DateTime<Utc>,
    pub slug: String,
    pub draft_content: String,
}

impl Post {
    pub fn publish(
        &self,
        posts_path: &PostPath,
        translation_path: &TranslationPath,
    ) -> std::io::Result<(PostPath, TranslationPath)> {
        let post_file_name = self.build_file_name(posts_path);
        let content = self.build_content();
        let translation = &self.build_translation();

        // Запись и перевод
        let write_files = || -> std::io::Result<(PostPath, TranslationPath)> {
            fs::write(&post_file_name, content)?;

            File::options()
                .append(true)
                .open(translation_path)?
                .write_all(translation.as_bytes())?;

            let published_post_path = PostPath::from(posts_path).join(&post_file_name);

            Ok((published_post_path, TranslationPath::from(translation_path)))
        };

        write_files()
    }

    fn build_content(&self) -> String {
        serialize_with_template(self, POST_TEMPLATE.to_string())
    }

    fn build_file_name(&self, posts_path: &Path) -> String {
        let lang = self.lang.to_lowercase();
        let posts_path = posts_path.to_str().unwrap();
        format!(
            "{posts_path}/{}-{}@{lang}.md",
            self.published_date_time.format(ISO8601_DATE_FORMAT),
            self.slug,
        )
    }

    fn build_translation(&self) -> String {
        process_template(
            TRANSLATION_TEMPLATE.to_owned(),
            HashMap::from([("id", self.slug.clone()), ("value", self.title.clone())]),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::grow::lang::Lang;
    use crate::grow::post::Post;
    use crate::grow::{
        DEFAULT_AUTHOR, ISO8601_DATE_FORMAT, TEST_CONTENT, TEST_DESCRIPTION, TEST_DRAFT_TITLE,
        TEST_SLUG,
    };
    use chrono::{DateTime, Utc};

    fn generate_fake_post(predefined_date_time: &DateTime<Utc>) -> Post {
        Post {
            title: String::from(TEST_DRAFT_TITLE),
            author: String::from(DEFAULT_AUTHOR),
            description: String::from(TEST_DESCRIPTION),
            lang: Lang::Ru,
            slug: String::from(TEST_SLUG),
            keywords: vec!["бумага".to_string(), "А4".to_string(), "297 мм".to_string()],
            published_date_time: *predefined_date_time,
            draft_content: String::from(TEST_CONTENT),
        }
    }

    #[test]
    fn test_post_to_string() {
        let predefined_date_time = Utc::now();
        let post = generate_fake_post(&predefined_date_time);

        let grow_post_as_string = post.build_content();
        assert!(!grow_post_as_string.is_empty());

        dbg!(&grow_post_as_string);

        // title это slug идентификатор для системы перевода. title из post будет использован в переводе
        let expected_title_is_slug = format!("${}@: {}", "title", &post.slug);
        assert!(
            grow_post_as_string.contains(&expected_title_is_slug),
            "exp = {}",
            expected_title_is_slug
        );

        let expected_author = format!("{}@: {}", "author", &post.author);
        assert!(
            grow_post_as_string.contains(&expected_author),
            "exp = {}",
            expected_author
        );

        let expected_description = format!("{}: {}", "description", &post.description);
        assert!(
            grow_post_as_string.contains(&expected_description),
            "exp = {}",
            expected_description
        );

        let expected_slug = format!("slug{}: {}", &post.lang, &post.slug);
        assert!(
            grow_post_as_string.contains(&expected_slug),
            "exp = {}",
            expected_slug
        );

        let formatted_naive_date_time = predefined_date_time.format(ISO8601_DATE_FORMAT);

        let expected_publish_date = format!("$dates:\n  published: {}", &formatted_naive_date_time);
        assert!(
            grow_post_as_string.contains(&expected_publish_date),
            "exp = {}",
            expected_publish_date
        );

        let expected_content = &post.build_content();
        assert!(
            grow_post_as_string.contains(expected_content),
            "exp = {}",
            expected_content
        );
    }
}
