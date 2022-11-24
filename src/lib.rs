extern crate core;

use crate::commands::{CommandResult, Run};
use crate::commands::{INDEX_COMMAND_NAME, PUBLISH_COMMAND_NAME, HELP_COMMAND_NAME};
use crate::commands::index::IndexCommand;
use crate::commands::publish::PublishCommand;
use crate::commands::unknown::UnknownCommand;

pub mod commands;

pub fn run<'a>(raw_command: &'a str, params: &'a [String]) -> Result<CommandResult<'a>, String> {
    match raw_command {
        INDEX_COMMAND_NAME => IndexCommand.run(),
        PUBLISH_COMMAND_NAME => PublishCommand.run(),
        HELP_COMMAND_NAME => Ok(
            CommandResult {
                command: "help",
                details: "".to_string(),
            }), // / TODO add Help command
        _unknown => UnknownCommand.run()
    }
}

#[cfg(test)]
mod tests {
    use crate::{run, CommandResult};
    use crate::commands::{available_commands, Command};
    use crate::commands::unknown::UNKNOWN_MESSAGE_ERROR;

    #[test]
    fn test_run_index_command() {
        let expected_command_result = CommandResult {
            command: "index",
            details: String::new(),
        };

        let result = run(expected_command_result.command, &[]).unwrap();
        assert_eq!(expected_command_result.summarize(), result.summarize());
    }

    #[test]
    #[should_panic]
    fn test_run_unknown_command() {
        let available_commands = available_commands().join(",");
        let resolved_error_message = UNKNOWN_MESSAGE_ERROR.replace("{commands}", available_commands.as_str());

        run("unknown", &[])
            .expect(
                format!("{}", resolved_error_message).as_str(),
            );
    }
}
