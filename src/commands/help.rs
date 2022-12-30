use crate::commands::{CommandResult, Error, MashinkaCommand, HELP_COMMAND_NAME};
use std::collections::HashMap;

pub struct HelpCommand;

impl HelpCommand {
    pub fn new() -> Box<HelpCommand> {
        Box::new(Self)
    }
}

impl MashinkaCommand for HelpCommand {
    fn run(&self) -> Result<CommandResult, Error> {
        Ok(CommandResult {
            command: HELP_COMMAND_NAME.to_string(),
            details: HashMap::new(),
        })
    }
}
