#![allow(clippy::must_use_candidate)]

use crate::command::Error;
use crate::grow::lang::Lang;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub const PARAMETER_KEY_VALUE_DELIMITER: &str = "=";
pub const PARAMETER_PREFIX: &str = "--";

pub struct Config {
    args_map: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let args_map: HashMap<String, String> = HashMap::new();
        Self { args_map }
    }
}

impl Config {
    pub fn parse_args(args: impl Iterator<Item = String>) -> Result<Self, Error> {
        let mut args_map = HashMap::new();
        for param in args {
            if param.contains(PARAMETER_KEY_VALUE_DELIMITER) {
                let (k, v) = param.split_once(PARAMETER_KEY_VALUE_DELIMITER).unwrap();
                args_map.insert(k.to_string(), v.to_string());
                continue;
            }

            if param.contains(PARAMETER_PREFIX) {
                args_map.insert(param, String::from("true"));
                continue;
            }
            return Err(Error::Parse());
        }

        Ok(Self { args_map })
    }

    pub fn is_dry_run(&self) -> bool {
        self.args_map.contains_key("--dry-run")
    }

    /// Возвращает путь до grow черновика
    /// Если задан параметр --draft-path, то использует его, иначе берет значение из переменной
    /// окружения `ABS_POST_DRAFT_FILE`.
    pub fn get_draft_path_or_default(&self) -> Result<PathBuf, Error> {
        let default_draft_path = env::var("ABS_POST_DRAFT_FILE")?;

        if default_draft_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_POST_DRAFT_FILE")));
        }

        let draft_path = self
            .args_map
            .get("--draft-path")
            .unwrap_or(&default_draft_path);

        Ok(PathBuf::from(draft_path))
    }

    /// Возвращает путь до post записей
    /// Если задан параметр `--posts-path`, то использует его, иначе берет значение из переменной
    /// окружения `ABS_POSTS_PATH`.
    pub fn get_posts_path_or_default(&self, lang: Lang) -> Result<PathBuf, Error> {
        let default_posts_path = env::var("ABS_POSTS_PATH")?;

        if default_posts_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_POSTS_PATH")));
        }

        let resolved_path = self
            .args_map
            .get("--posts-path")
            .unwrap_or(&default_posts_path)
            .replace("[lang]", &lang.to_lowercase());

        Ok(PathBuf::from(&resolved_path))
    }

    /// Возвращает путь до переводов в зависимости от языка записи.
    /// Если задан параметр --translations-path, то использует его, иначе берет значение из
    /// переменной окружения `ABS_TRANSLATIONS_PATH`.
    pub fn get_translation_path_or_default(&self, lang: Lang) -> Result<PathBuf, Error> {
        let default_translation_path = env::var("ABS_TRANSLATIONS_PATH")?;

        if default_translation_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_TRANSLATIONS_PATH")));
        }

        let resolved_path = self
            .args_map
            .get("--translations-path")
            .unwrap_or(&default_translation_path)
            .replace("[lang]", &lang.to_lowercase());

        Ok(PathBuf::from(&resolved_path))
    }
}
