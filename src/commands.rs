pub mod index;
pub mod publish;
pub mod unknown;

pub const INDEX_COMMAND_NAME: &str = "index";
pub const PUBLISH_COMMAND_NAME: &str = "publish";
pub const HELP_COMMAND_NAME: &str = "help";

pub fn available_commands() -> [&'static str; 3] {
    [INDEX_COMMAND_NAME, PUBLISH_COMMAND_NAME, HELP_COMMAND_NAME]
}

pub trait Run {
    fn run(&self) -> Result<CommandResult, String>;
}

pub struct CommandResult<'a> {
    pub command: &'a str,
    pub details: String,
}

impl<'a> CommandResult<'a> {
    pub fn summarize(&self) -> String {
        format!(
            "Command {} successfully completed. Details {}", &self.command, &self.details
        )
    }
}