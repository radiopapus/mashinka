#![allow(clippy::must_use_candidate)]

use crate::command::help::Help;
use crate::command::publish::Publish;
use crate::config::Config;
use std::collections::HashMap;
use std::env;
use thiserror::Error;

pub mod help;
pub mod index;
pub mod publish;

pub const INDEX_COMMAND_NAME: &str = "index";
pub const PUBLISH_COMMAND_NAME: &str = "publish";
pub const HELP_COMMAND_NAME: &str = "help";

pub fn available_commands() -> [&'static str; 3] {
    [INDEX_COMMAND_NAME, PUBLISH_COMMAND_NAME, HELP_COMMAND_NAME]
}

#[derive(Error, Debug)]
pub enum Error {
    // config
    #[error("Check parameter format, please. Should be --param-name or --param-name=value")]
    Parse(),
    #[error("Value for {0} should not be empty")]
    EmptyValue(String),
    #[error("test")]
    EnvVar(#[from] env::VarError),

    // deserializer
    #[error("Have no clue about {0} key")]
    UnknownKey(String),
    #[error("Have no clue about {0} language value")]
    UnknownLang(String),
    #[error("Can't read file {0}")]
    ReadDraft(std::io::Error),
    #[error("Can't write file {0}")]
    WritePost(std::io::Error),
    //Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // }
}

pub trait Command {
    fn run(&self) -> Result<CommandResult, Error>;
}

pub struct CommandResult {
    command: String,
    details: HashMap<String, String>,
}

impl CommandResult {
    pub fn summarize(&mut self) -> String {
        let details = if self.details.is_empty() {
            String::new()
        } else {
            format!("Details: {:#?}", self.details)
        };

        format!(
            "Command `{}` successfully completed. {}",
            self.command, details
        )
    }
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<CommandResult, Error> {
    let command = match args.next() {
        Some(v) => v,
        None => String::from(HELP_COMMAND_NAME),
    };

    let config = Config::parse_args(args)?;

    run_with_config(&command, config)
}

fn run_with_config(command: &str, config: Config) -> Result<CommandResult, Error> {
    let cmd: Box<dyn Command> = match command {
        // INDEX_COMMAND_NAME => IndexCommand.run(config),
        PUBLISH_COMMAND_NAME => Publish::new(config),
        HELP_COMMAND_NAME => Help::new(),
        _unknown => {
            let available_commands = available_commands();
            panic!(
                "Unknown command {}. Available command are {:?} or type `mashinka help` for help",
                command, available_commands
            );
        }
    };

    cmd.run()
}

#[cfg(test)]
mod tests {
    use crate::command::{available_commands, run, CommandResult, HELP_COMMAND_NAME};
    use std::collections::HashMap;

    #[test]
    fn test_run_command() {
        let mut expected_command_result = CommandResult {
            command: HELP_COMMAND_NAME.to_string(),
            details: HashMap::new(),
        };

        let mut result = run([HELP_COMMAND_NAME.to_string()].into_iter()).unwrap();
        assert_eq!(expected_command_result.summarize(), result.summarize());
    }

    #[test]
    #[should_panic]
    fn test_run_unknown_command() {
        let available_commands = available_commands().join(",");
        let resolved_error_message = "Unknown command {command}. \
        Available command are {command} or type `mashinka help` for help"
            .replace("{command}", "unknown")
            .replace("{command}", available_commands.as_str());

        run(["unknown".to_string()].into_iter())
            .unwrap_or_else(|_| panic!("{}", resolved_error_message));
    }
}
