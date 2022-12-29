use crate::grow::lang::Lang;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Check parameter format, please. Should be --param-name or --param-name=value")]
    Parse(),
    #[error("Value for {0} should not be empty")]
    EmptyValue(&'a str),
    #[error("test")]
    EnvVar(#[from] env::VarError),
    //Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // }
}

impl Config {
    pub fn parse_args<'a>(args: impl Iterator<Item = String>) -> Result<Self, Error<'a>> {
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
    pub fn get_draft_path_or_default<'a>(&self) -> Result<PathBuf, Error<'a>> {
        let default_draft_path = env::var("ABS_POST_DRAFT_FILE")?;

        if default_draft_path.is_empty() {
            return Err(Error::EmptyValue("ABS_POST_DRAFT_FILE"));
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
    pub fn get_posts_path_or_default<'a>(&self, lang: Lang) -> Result<PathBuf, Error<'a>> {
        let default_posts_path = env::var("ABS_POSTS_PATH")?;

        if default_posts_path.is_empty() {
            return Err(Error::EmptyValue("ABS_POSTS_PATH"));
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
    pub fn get_translation_path_or_default<'a>(&self, lang: Lang) -> Result<PathBuf, Error<'a>> {
        let default_translation_path = env::var("ABS_TRANSLATIONS_PATH")?;

        if default_translation_path.is_empty() {
            return Err(Error::EmptyValue("ABS_TRANSLATIONS_PATH"));
        }

        let resolved_path = self
            .args_map
            .get("--translations-path")
            .unwrap_or(&default_translation_path)
            .replace("[lang]", &lang.to_lowercase());

        Ok(PathBuf::from(&resolved_path))
    }
}
