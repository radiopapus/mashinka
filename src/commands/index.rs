use crate::commands::{CommandResult, MashinkaCommand, INDEX_COMMAND_NAME};
use std::collections::HashMap;
use std::error::Error;

pub struct IndexCommand;

/// Индексирует записи для поиска
impl MashinkaCommand for IndexCommand {
    fn run(&self) -> Result<CommandResult, Box<dyn Error>> {
        Ok(CommandResult {
            command: INDEX_COMMAND_NAME.to_string(),
            details: HashMap::new(),
        })
    }
}
