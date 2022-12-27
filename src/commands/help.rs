use crate::commands::{CommandResult, MashinkaCommand, HELP_COMMAND_NAME};
use std::error::Error;

pub struct HelpCommand;

impl HelpCommand {
    pub fn new() -> Box<HelpCommand> {
        Box::new(Self)
    }
}

impl MashinkaCommand for HelpCommand {
    fn run(&self) -> Result<CommandResult, Box<dyn Error>> {
        Ok(CommandResult {
            command: HELP_COMMAND_NAME.to_string(),
            details: String::new(),
        })
    }
}
