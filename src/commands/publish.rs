use crate::commands::{CommandResult, PUBLISH_COMMAND_NAME, Run};

pub struct PublishCommand;

impl Run for PublishCommand {
    fn run(&self,params: impl Iterator<Item = String>) -> Result<CommandResult, String> {
        Ok(
            CommandResult {
                command: PUBLISH_COMMAND_NAME,
                details: String::new(),
            }
        )
    }
}