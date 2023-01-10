#![allow(clippy::must_use_candidate)]

use crate::command::{Command, CommandResult, Details, Error, HELP_COMMAND_NAME};

pub struct Help;

impl Help {
    pub fn new() -> Box<Help> {
        Box::new(Self)
    }
}

impl Command for Help {
    fn run(&self) -> Result<CommandResult, Error> {
        Ok(CommandResult {
            command: HELP_COMMAND_NAME.to_string(),
            details: Details::new()
        })
    }
}
