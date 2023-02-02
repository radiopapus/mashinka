#![allow(clippy::must_use_candidate)]
#![allow(clippy::or_fun_call)]

use std::collections::HashMap;
use regex::RegexBuilder;
use crate::command::{Command, CommandResult, Details, Error, INDEX_COMMAND_NAME};
use crate::config::Config;
use crate::grow::post::{GrowPost, GrowPostTranslation, WriterWrapper};
use crate::grow::serdes::{process_template};
use crate::grow::TRANSLATION_INDEX_TEMPLATE;

pub struct Index {
    config: Config
}

impl Index {
    pub fn new(config: Config) -> Box<Index> {
        Box::new(Self { config })
    }
}

/// Данные для индексатора
#[derive(Debug, Default)]
pub struct IndexContent {
    /// Путь до записи относительно корня сайта, например /ru/posts/{post_name}
    id: String,
    /// Заголовок на языке записи
    title: String,
    /// Текст записи для индексации
    content: String,
}

impl IndexContent {
    /// Удаляем все ненужное (html, переносы строк, табы), экранируем кавычки
    /// и оставляем нужное (шутка) только текст.
    pub fn sanitize(value: &str) -> String {
        let re = RegexBuilder::new(r#"<[^>]*>"#).build().unwrap();
        re.replace_all(value, "")
            .replace(['\n', '\r', '\t', '\u{a0}', '\\' ], "")
            .replace('"', "\\\"")
    }

    pub fn from_post_and_translation(post: &GrowPost, translated_value: &str) -> IndexContent {
        IndexContent {
            id: format!("/{}/posts/{}", post.lang.to_lowercase(), post.slug),
            title: Self::sanitize(translated_value),
            content: Self::sanitize(post.text.as_str())
        }
    }
}

impl ToString for IndexContent {
    fn to_string(&self) -> String {
        process_template(
            TRANSLATION_INDEX_TEMPLATE.to_string(), HashMap::from([
                ("id", self.id.clone()),
                ("title", self.title.clone()),
                ("content", self.content.clone())
            ])
        )
    }
}

/// Индексирует записи для поиска
impl Command for Index {
    fn run(&self) -> Result<CommandResult, Error> {
        let config = &self.config;

        let mut all_translations: Vec<GrowPostTranslation> = Vec::new();

        // Собираем все переводы
        for lang in config.available_languages() {
            let translation_path = config.get_translations_path_or_default()?
                .join(lang.to_lowercase())
                .join("LC_MESSAGES/messages.po");

            let translations = GrowPostTranslation::fetch_translations(&translation_path)?;
            all_translations = [all_translations, translations].concat();
        }

        let translation_map: HashMap<String, String> = all_translations.into_iter()
            .map(|t| (t.id, t.translated_value))
            .collect();

        let mut index_content_items = vec![];
        let posts_path = config.get_posts_path_or_default()?;

        // Сопоставляем translation post и добавляем в index_content_items для записи в индекс.
        for lang in config.available_languages() {
            let posts = GrowPost::fetch_posts_by_lang(&posts_path, lang)?;

            for grow_post in posts {
                let translation = translation_map.get(&grow_post.slug).ok_or(
                    Error::IncorrectFormat(format!("slug not found in translation `{:?}`", &grow_post))
                )?;

                let index_content = IndexContent::from_post_and_translation(&grow_post, translation);
                index_content_items.push(index_content.to_string());
            }
        }

        let index_path = &config.get_index_file_path_or_default()?;
        let mut details = Details::new();

        let as_json_list = format!(r#"[{}]"#, index_content_items.join(","));
        details.push(String::from("index_path"), index_path.to_string_lossy().to_string());

        let command = String::from(INDEX_COMMAND_NAME);

        if config.is_dry_run() {
            return Ok(CommandResult { command, details })
        }

        WriterWrapper::write_file(index_path, &as_json_list)?;

        Ok(CommandResult { command, details })
    }
}
