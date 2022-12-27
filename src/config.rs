use crate::grow::lang::Lang;
use crate::grow::DEFAULT_LANG;
use std::collections::HashMap;
use std::env;
use std::path::Path;

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
    pub fn parse_args(args: impl Iterator<Item = String>) -> Self {
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
            panic!("Check parameter format, please. Should be --param-name or --param-name=value")
        }

        Self {
            args_map,
            ..Self::default()
        }
    }

    pub fn is_dry_run(&self) -> bool {
        self.args_map.contains_key("--dry-run")
    }

    /// Возвращает путь до grow черновика
    /// Если задан параметр --draft-path, то использует его, иначе берет значение из переменной
    /// окружения REL_POST_DRAFT_FILE.
    pub fn get_draft_path_or_default(&self) -> Box<Path> {
        let default_draft_path = env::var("ABS_POST_DRAFT_FILE").unwrap();

        let draft_path = self
            .args_map
            .get("--draft-path")
            .unwrap_or(&default_draft_path);

        Box::from(Path::new(draft_path))
    }

    /// Возвращает путь до post записей
    /// Если задан параметр --posts-path, то использует его, иначе берет значение из переменной
    /// окружения REL_POSTS_PATH.
    pub fn get_posts_path_or_default(&self, lang: Lang) -> Box<Path> {
        let default_posts_path = env::var("ABS_POSTS_PATH").unwrap();

        let resolved_path = self
            .args_map
            .get("--posts-path")
            .unwrap_or(&default_posts_path)
            .replace("[lang]", &lang.to_lowercase());

        Box::from(Path::new(&resolved_path))
    }

    /// Возвращает путь до переводов в зависимости от языка записи.
    /// Если задан параметр --translations-path, то использует его, иначе берет значение из
    /// переменной окружения REL_TRANSLATIONS_PATH.
    pub fn get_translation_path_or_default(&self, lang: Lang) -> Box<Path> {
        let default_translation_path = env::var("ABS_TRANSLATIONS_PATH").unwrap();

        let resolved_path = self
            .args_map
            .get("--translations-path")
            .unwrap_or(&default_translation_path)
            .replace("[lang]", &lang.to_lowercase());

        Box::from(Path::new(&resolved_path))
    }
}
