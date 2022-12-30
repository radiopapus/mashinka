use crate::commands::{CommandResult, Error, MashinkaCommand, INDEX_COMMAND_NAME};
use std::collections::HashMap;

pub struct IndexCommand;

/// Индексирует записи для поиска
impl MashinkaCommand for IndexCommand {
    fn run(&self) -> Result<CommandResult, Error> {
        Ok(CommandResult {
            command: INDEX_COMMAND_NAME.to_string(),
            details: HashMap::new(),
        })
    }
}
