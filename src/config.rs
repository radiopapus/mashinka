#![allow(clippy::must_use_candidate)]
#![allow(clippy::or_fun_call)]

use crate::command::Error;
use crate::grow::lang::Lang;
use std::collections::HashMap;
use std::env;
use std::env::Args;
use std::path::PathBuf;

pub const VERSION: &str = "0.1.4";

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

pub struct DeployConfig {
    pub account_id: String,
    pub username: String,
    pub password: String,
    pub container_id: String,
    pub destination: String,
}

impl ToString for DeployConfig {
    fn to_string(&self) -> String {
        unimplemented!("DeployConfig contains sensitive data!")
    }
}

//todo WHY fn in Config is not static?
impl Config {
    pub fn available_languages(&self) -> Vec<Lang> {
        vec![Lang::Ru, Lang::En]
    }

    /// # Errors
    ///
    /// Вернет `Error` если при парсинге входных параметров ключи имеют неверный формат.
    /// Они должны иметь формат --key-name или --key-name=value.
    pub fn from_args(args: Args) -> Result<Self, Error> {
        let mut args_map = HashMap::new();
        for param in args {
            if param.contains(PARAMETER_KEY_VALUE_DELIMITER) {
                let (k, v) = param.split_once(PARAMETER_KEY_VALUE_DELIMITER).ok_or(
                    Error::IncorrectFormat(format!(
                        "Parameter key values should be delimited by {}",
                        PARAMETER_KEY_VALUE_DELIMITER
                    )),
                )?;
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
    ///
    /// # Errors
    ///
    /// Вернет `Error` если переменная окружения `ABS_POST_DRAFT_FILE` не задана или имеет нулевую длинну.
    pub fn get_draft_path_or_default(&self) -> Result<PathBuf, Error> {
        let default_draft_path = env::var("ABS_POST_DRAFT_FILE")?;

        if default_draft_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_POST_DRAFT_FILE")));
        }

        let draft_path = self.args_map.get("--draft-path")
            .unwrap_or(&default_draft_path);

        Ok(PathBuf::from(draft_path))
    }

    /// Возвращает путь до post записей
    /// Если задан параметр `--posts-path`, то использует его, иначе берет значение из переменной
    /// окружения `ABS_POSTS_PATH`.
    /// # Errors
    ///
    /// Вернет `Error` если переменная окружения `ABS_POSTS_PATH` не задана или имеет нулевую длину.
    pub fn get_posts_path_or_default(&self) -> Result<PathBuf, Error> {
        let default_posts_path = env::var("ABS_POSTS_PATH")?;

        if default_posts_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_POSTS_PATH")));
        }

        let path = self.args_map.get("--posts-path")
            .unwrap_or(&default_posts_path);

        Ok(PathBuf::from(&path))
    }

    /// Возвращает путь до переводов в зависимости от языка записи.
    /// Если задан параметр --translations-path, то использует его, иначе берет значение из
    /// переменной окружения `ABS_TRANSLATIONS_PATH`.
    /// # Errors
    ///
    /// Вернет Error если переменная окружения `ABS_TRANSLATIONS_PATH` не задана или имеет нулевую длину.
    pub fn get_translations_path_or_default(&self) -> Result<PathBuf, Error> {
        let default_translation_path = env::var("ABS_TRANSLATIONS_PATH")?;

        if default_translation_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_TRANSLATIONS_PATH")));
        }

        let path = self.args_map.get("--translations-path")
            .unwrap_or(&default_translation_path);

        Ok(PathBuf::from(&path))
    }

    /// Возвращает путь до файла с индексом в зависимости от языка записи.
    /// Если задан параметр --index-path, то использует его, иначе берет значение из
    /// переменной окружения `ABS_INDEX_FILE`.
    /// # Errors
    ///
    /// Вернет Error если переменная окружения `ABS_INDEX_FILE` не задана или имеет нулевую длину.
    pub fn get_index_file_path_or_default(&self) -> Result<PathBuf, Error> {
        let default_path = env::var("ABS_INDEX_FILE")?;

        if default_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_INDEX_FILE")));
        }

        let resolved_path = self.args_map.get("--index-path")
            .unwrap_or(&default_path);

        Ok(PathBuf::from(&resolved_path))
    }

    /// Возвращает путь до директории, в которой лежат подготовленные для выгрузки данные.
    /// Если задан параметр --build-path, то использует его, иначе берет значение из
    /// переменной окружения `ABS_BUILD_PATH`.
    /// # Errors
    ///
    /// Вернет Error если переменная окружения `ABS_BUILD_PATH` не задана или имеет нулевую длину.
    pub fn get_build_path_or_default(&self) -> Result<PathBuf, Error> {
        let default_path = env::var("ABS_BUILD_PATH")?;

        if default_path.is_empty() {
            return Err(Error::EmptyValue(String::from("ABS_BUILD_PATH")));
        }

        let resolved_path = self.args_map.get("--build-path")
            .unwrap_or(&default_path);

        Ok(PathBuf::from(&resolved_path))
    }

    /// Возвращает Deploy config, который содержит реквизиты для доступа к облачному хранилищу и выполнения деплоя.
    pub fn get_deploy_config(&self) -> Result<DeployConfig, Error> {
        let account_id = env::var("DEPLOY_ACCOUNT")?;
        let username = env::var("DEPLOY_USERNAME")?;
        let password = env::var("DEPLOY_PASSWORD")?;
        let container_id = env::var("DEPLOY_CONTAINER")?;
        let destination = env::var("DEPLOY_DEST")?;

        if account_id.is_empty() || username.is_empty() || password.is_empty() || container_id.is_empty() || destination.is_empty() {
            return Err(
                Error::EmptyValue("Check deploy vars: DEPLOY_ACCOUNT, DEPLOY_USERNAME, DEPLOY_PASSWORD, DEPLOY_CONTAINER, DEPLOY_DEST".to_string())
            );
        }

        Ok(DeployConfig{ account_id, username, password, container_id, destination })
    }
}
