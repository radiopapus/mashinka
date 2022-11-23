use crate::commands::{Command, CommandResult, Run};

pub struct UnknownCommand;

pub static UNKNOWN_MESSAGE_ERROR: &str = "Unknown command. Available commands are {commands} or type `mashinka help` for help";

impl Run for UnknownCommand {

    fn run(&self) -> Result<CommandResult, String> {
        let available_commands = Command::available_commands().join(",");
        Err(
            format!("{}", UNKNOWN_MESSAGE_ERROR.replace("{commands}", available_commands.as_str())),
        )
    }
}