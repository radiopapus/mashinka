#![allow(clippy::must_use_candidate)]
#![allow(clippy::or_fun_call)]

use std::{env, fs};
use std::fs::File;
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::command::{Command, CommandResult, DEPLOY_COMMAND_NAME, Details, Error};
use crate::config::{Config, DeployConfig};

pub struct Deploy {
    config: Config
}

impl Deploy {
    pub fn new(config: Config) -> Box<Deploy> {
        Box::new(Self { config })
    }
}

/// Создает tar.gz указанной директории.
fn encode_dir(archive_path: &PathBuf, dir_path: &Path) -> Result<(), Error> {
    let tar_gz = File::create(archive_path).map_err(Error::CreateFile)?;
    let enc = GzEncoder::new(tar_gz, Compression::best());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", dir_path).map_err(Error::CreateArchive)?;
    tar.finish().map_err(Error::CreateArchive)?;
    Ok(())
}

/// Получает токен для работы с API Selectel.
fn fetch_token(deploy_config: &DeployConfig) -> Result<String, Error> {
    let response = ureq::get("https://api.selcdn.ru/auth/v1.0")
        .set("X-Auth-User", deploy_config.username.as_str())
        .set("X-Auth-Key", deploy_config.password.as_str())
        .call()
        .map_err(|e| Error::DeployApi(e.to_string()))?;

    let token = response.header("X-Storage-Token")
        .ok_or(Error::DeployApi("Key X-Storage-Token does not exists".to_string()))?;

    Ok(String::from(token))
}

/// Выгружает архив в хранилище и распаковывает его в корень.
fn upload_and_extract(archive: &PathBuf, deploy_config: &DeployConfig, token: &str) -> Result<(), Error> {
    let account_id = deploy_config.account_id.as_str();
    let container_id = deploy_config.container_id.as_str();

    let endpoint = format!("https://api.selcdn.ru/v1/SEL_{account_id}/{container_id}/");
    let content = fs::read(archive).map_err(Error::ReadFile)?;

    ureq::put(endpoint.as_str())
        .set("X-Auth-Token", token)
        .query("extract-archive", "tar.gz")
        .send_bytes(content.as_slice())
        .map_err(|e| Error::DeployApi(e.to_string()))?;

    Ok(())
}

/// Выгружает данные в облачное хранилище
/// // архивировать build в tar.gz
/// получить токен от облачного хранилища
/// сгенерить запрос к API, прикрепить архив tar.gz, указать флаг на распаковку данных после выгрузки
/// curl -i -XPUT  https://api.selcdn.ru/v1/SEL_*****/new_container/archive.tar.gz/?extract-archive=tar.gz \
/// -H "X-Auth-Token: $token" -T "archive.tar.gz"
/// сделать запрос на главную и вывести на экран или в открыть в браузере
impl Command for Deploy {
    fn run(&self) -> Result<CommandResult, Error> {
        let config = &self.config;
        let grow_build_path = &config.get_build_path_or_default()?;

        let archive_path = env::temp_dir().join(Path::new("build.tar.gz"));
        encode_dir(&archive_path, grow_build_path)?;

        let mut details = Details::new();
        details.push(String::from("deployed_to"), config.get_deploy_config()?.destination);

        let command = String::from(DEPLOY_COMMAND_NAME);

        if config.is_dry_run() {
            return Ok(CommandResult { command, details })
        }

        let deploy_config  = config.get_deploy_config()?;
        let token = fetch_token(&deploy_config)?;
        upload_and_extract(&archive_path, &deploy_config, token.as_str())?;

        Ok(CommandResult { command, details })
    }
}
