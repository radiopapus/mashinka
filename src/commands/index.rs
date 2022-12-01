use crate::commands::{CommandResult, Run};

pub struct IndexCommand;

/// Implementation for the index command.
/// Gets content and build index to use it for search.
impl Run for IndexCommand {
    fn run(&self, params: impl Iterator<Item = String>) -> Result<CommandResult, String> {
        Ok(
            CommandResult {
                command: "index",
                details: String::new(),
            }
        )
    }
}
