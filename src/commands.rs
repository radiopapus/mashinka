use std::iter;
use crate::commands::index::IndexCommand;
use crate::commands::publish::PublishCommand;

pub mod index;
pub mod publish;

pub const INDEX_COMMAND_NAME: &str = "index";
pub const PUBLISH_COMMAND_NAME: &str = "publish";
pub const HELP_COMMAND_NAME: &str = "help";

pub fn available_commands() -> [&'static str; 3] {
    [INDEX_COMMAND_NAME, PUBLISH_COMMAND_NAME, HELP_COMMAND_NAME]
}

pub trait Run {
    fn run(&self, params: impl Iterator<Item=String>) -> Result<CommandResult, String>;
}

pub struct CommandResult<'a> {
    command: &'a str,
    details: String,
}

impl<'a> CommandResult<'a> {
    pub fn summarize(&self) -> String {
        format!(
            "Command {} successfully completed. Details {}", &self.command, &self.details
        )
    }
}

pub fn run(command: &str) -> Result<CommandResult, String> {
    run_with_params(command, iter::empty::<String>())
}

pub fn run_with_params(command: &str, params: impl Iterator<Item=String>) -> Result<CommandResult, String> {
    match command {
        INDEX_COMMAND_NAME => IndexCommand.run(params),
        PUBLISH_COMMAND_NAME => PublishCommand.run(params),
        HELP_COMMAND_NAME => Ok(
            CommandResult {
                command,
                details: "".to_string(),
            }),
        _unknown => {
            let available_commands = available_commands();
            panic!(
                "Unknown command {}. Available commands are {:?} or type `mashinka help` for help",
                command, available_commands
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{run};
    use crate::commands::{available_commands, CommandResult, HELP_COMMAND_NAME};

    #[test]
    fn test_run_command() {
        let expected_command_result = CommandResult {
            command: HELP_COMMAND_NAME,
            details: String::new(),
        };

        let result = run(expected_command_result.command).unwrap();
        assert_eq!(expected_command_result.summarize(), result.summarize());
    }

    #[test]
    #[should_panic]
    fn test_run_unknown_command() {
        let available_commands = available_commands().join(",");
        let resolved_error_message = "Unknown command {command}. \
        Available commands are {commands} or type `mashinka help` for help"

        .replace("{command}", "unknown")
        .replace("{commands}", available_commands.as_str());

        run("unknown")
            .expect(format!("{}", resolved_error_message).as_str());
    }
}
