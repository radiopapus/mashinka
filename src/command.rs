#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]

use crate::command::help::Help;
use crate::command::publish::Publish;
use crate::config::Config;
use std::{env};
use std::env::Args;
use std::fmt::{Display, Formatter};
use chrono::ParseError;
use thiserror::Error;
use crate::command::deploy::Deploy;
use crate::command::index::Index;
use crate::command::version::Version;

pub mod help;
pub mod index;
pub mod publish;
pub mod deploy;
pub mod version;

pub const INDEX_COMMAND_NAME: &str = "index";
pub const PUBLISH_COMMAND_NAME: &str = "publish";
pub const HELP_COMMAND_NAME: &str = "help";
pub const DEPLOY_COMMAND_NAME: &str = "deploy";
pub const VERSION_COMMAND_NAME: &str = "version";

/// Список ошибок
#[derive(Error, Debug)]
pub enum Error {
    // config
    #[error("Check parameter format, please. Should be --param-name or --param-name=value")]
    Parse(),
    #[error("Check date time format `{0}`")]
    DateTimeError(ParseError),
    #[error("Value for {0} should be filled (not empty)")]
    EmptyValue(String),
    #[error("Value for {0} is too long, example: {1}. Expected less than {2}")]
    ValueTooLong(String, String, usize),
    #[error("Env variable {0:?}")]
    EnvVar(#[from] env::VarError),
    // deserializer
    #[error("Have no clue how process about {0} key")]
    UnknownKey(String),
    #[error("Have no clue how to process {0} language value")]
    UnknownLang(String),
    #[error("Can't create file {0:?}")]
    CreateFile(std::io::Error),
    #[error("Can't create archive {0:?}")]
    CreateArchive(std::io::Error),
    #[error("Error deploy api archive {0:?}")]
    DeployApi(String),
    #[error("Can't read file {0:?}")]
    ReadFile(std::io::Error),
    #[error("Can't write file {0:?}")]
    WriteFile(std::io::Error),
    #[error("Can't read dir {0:?}")]
    ReadDir(std::io::Error),
    #[error("Incorrect format. {0:?}")]
    IncorrectFormat(String)
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

pub trait Command {
    fn run(&self) -> Result<CommandResult, Error>;
}

#[derive(Debug, Default)]
pub struct Detail {
    pub id: String,
    pub message: String
}

#[derive(Debug, Default)]
pub struct Details {
    items: Vec<Detail>
}

impl Details {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, id: String, message: String) {
        self.items.push(Detail { id, message })
    }
}

impl Display for Details {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.items.is_empty() { return write!(f, "") }

        let msg: String = self.items.iter().map(|d| {
            if d.id.is_empty() { d.message.to_string() }
            else { format!("{}:{}", d.id, d.message) }
        }).collect();

        write!(f, "{}", msg)
    }
}

/// Результат выполнения команды
#[derive(Debug, Default)]
pub struct CommandResult {
    command: String,
    details: Details,
}

impl CommandResult {
    pub fn summarize(&mut self) -> String {
        if self.command.is_empty() { return format!("{}", self.details) }
        format!("Command `{}` successfully completed. {}", self.command, self.details)
    }
}

/// # Errors
///
/// Вернет Error при выполнении команды и парсинге конфигурации.
pub fn run(mut args: Args) -> Result<CommandResult, Error> {
    let command = match args.next() {
        Some(v) => v,
        None => String::from(HELP_COMMAND_NAME),
    };

    let config = Config::from_args(args)?;

    let cmd: Box<dyn Command> = match command.as_str() {
        INDEX_COMMAND_NAME => Index::new(config),
        PUBLISH_COMMAND_NAME => Publish::new(config),
        HELP_COMMAND_NAME => Help::new(),
        VERSION_COMMAND_NAME => Version::new(),
        DEPLOY_COMMAND_NAME => Deploy::new(config),
        _unknown => Help::new(),
    };

    cmd.run()
}
