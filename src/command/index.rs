use crate::command::{Command, CommandResult, Details, Error, INDEX_COMMAND_NAME};

pub struct Index;

/// Индексирует записи для поиска
impl Command for Index {
    fn run(&self) -> Result<CommandResult, Error> {
        Ok(CommandResult {
            command: INDEX_COMMAND_NAME.to_string(),
            details: Details::new()
        })
    }
}
