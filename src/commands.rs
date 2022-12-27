use crate::commands::help::HelpCommand;
use crate::commands::publish::PublishCommand;
use crate::config::Config;
use std::error::Error;

pub mod help;
pub mod index;
pub mod publish;

pub const INDEX_COMMAND_NAME: &str = "index";
pub const PUBLISH_COMMAND_NAME: &str = "publish";
pub const HELP_COMMAND_NAME: &str = "help";

pub fn available_commands() -> [&'static str; 3] {
    [INDEX_COMMAND_NAME, PUBLISH_COMMAND_NAME, HELP_COMMAND_NAME]
}

pub trait MashinkaCommand {
    fn run(&self) -> Result<CommandResult, Box<dyn Error>>;
}

pub struct CommandResult {
    command: String,
    details: String,
}

impl CommandResult {
    pub fn summarize(&self) -> String {
        let details = if self.details.is_empty() {
            String::new()
        } else {
            format!("Details: {}", self.details)
        };

        format!(
            "Command `{}` successfully completed. {}",
            self.command, details
        )
    }
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<CommandResult, Box<dyn Error>> {
    let command = match args.next() {
        Some(v) => v,
        None => String::from(HELP_COMMAND_NAME),
    };

    let config = Config::parse_args(args);

    run_with_config(&command, config)
}

fn run_with_config(command: &str, config: Config) -> Result<CommandResult, Box<dyn Error>> {
    let cmd: Box<dyn MashinkaCommand> = match command {
        // INDEX_COMMAND_NAME => IndexCommand.run(config),
        PUBLISH_COMMAND_NAME => PublishCommand::new(config),
        HELP_COMMAND_NAME => HelpCommand::new(),
        _unknown => {
            let available_commands = available_commands();
            panic!(
                "Unknown command {}. Available commands are {:?} or type `mashinka help` for help",
                command, available_commands
            );
        }
    };

    cmd.run()
}

#[cfg(test)]
mod tests {
    use crate::commands::{available_commands, run, CommandResult, HELP_COMMAND_NAME};

    #[test]
    fn test_run_command() {
        let expected_command_result = CommandResult {
            command: HELP_COMMAND_NAME.to_string(),
            details: String::new(),
        };

        let result = run([HELP_COMMAND_NAME.to_string()].into_iter()).unwrap();
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

        run(["unknown".to_string()].into_iter())
            .expect(format!("{}", resolved_error_message).as_str());
    }
}
