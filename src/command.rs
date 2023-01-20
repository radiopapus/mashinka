#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]

use crate::command::help::Help;
use crate::command::publish::Publish;
use crate::config::Config;
use std::{env};
use chrono::ParseError;
use thiserror::Error;
use crate::command::index::Index;

pub mod help;
pub mod index;
pub mod publish;

pub const INDEX_COMMAND_NAME: &str = "index";
pub const PUBLISH_COMMAND_NAME: &str = "publish";
pub const HELP_COMMAND_NAME: &str = "help";

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
    #[error("Env variable error")]
    EnvVar(#[from] env::VarError),
    // deserializer
    #[error("Have no clue how process about {0} key")]
    UnknownKey(String),
    #[error("Have no clue how to process {0} language value")]
    UnknownLang(String),
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
    /// # Errors
    ///
    /// Вернет Error при выполнении команды
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

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn push(&mut self, id: String, message: String) {
        self.items.push(Detail { id, message })
    }
}

/// Результат выполнения команды
pub struct CommandResult {
    command: String,
    details: Details,
}

impl CommandResult {
    pub fn summarize(&mut self) -> String {
        let details = if self.details.is_empty() {
            String::new()
        } else {
            format!("Details: {:#?}", self.details)
        };

        format!("Command `{}` successfully completed. {}", self.command, details)
    }
}

/// # Errors
///
/// Вернет Error при выполнении команды и парсинге конфигурации.
pub fn run(mut args: impl Iterator<Item = String>) -> Result<CommandResult, Error> {
    let command = match args.next() {
        Some(v) => v,
        None => String::from(HELP_COMMAND_NAME),
    };

    let config = Config::parse_args(args)?;

    let cmd: Box<dyn Command> = match command.as_str() {
        INDEX_COMMAND_NAME => Index::new(config),
        PUBLISH_COMMAND_NAME => Publish::new(config),
        HELP_COMMAND_NAME => Help::new(),
        _unknown => Help::new(),
    };

    cmd.run()
}

#[cfg(test)]
mod tests {
    use crate::command::{run, CommandResult, HELP_COMMAND_NAME, Details};

    #[test]
    fn test_run_command() {
        let mut expected_command_result = CommandResult {
            command: HELP_COMMAND_NAME.to_string(),
            details: Details::new(),
        };

        let mut result = run([HELP_COMMAND_NAME.to_string()].into_iter()).unwrap();
        assert_eq!(expected_command_result.summarize(), result.summarize());
    }
}
