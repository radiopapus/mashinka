use crate::commands::{CommandResult, Run};

pub struct PublishCommand;

impl Run for PublishCommand {
    fn run(&self) -> Result<CommandResult, String> {
        Ok(
            CommandResult {
                command: "publish",
                details: String::new(),
            }
        )
    }
}