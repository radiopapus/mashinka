use crate::CommandResult;
use crate::commands::Run;

pub struct IndexCommand;

/// Implementation fo the index command.
/// Gets content and build index to use it for search.
impl Run for IndexCommand {
    fn run(&self) -> Result<CommandResult, String> {

        Ok(
            CommandResult {
                command: "index",
                details: String::new(),
            }
        )
    }
}
