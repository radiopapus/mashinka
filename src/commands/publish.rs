use crate::commands::{CommandResult, MashinkaCommand, PUBLISH_COMMAND_NAME};
use crate::config::Config;
use crate::grow::serdes::grow_draft_deserializer::from_grow_draft_file;
use std::collections::HashMap;
use std::error::Error;

pub struct PublishCommand {
    config: Config,
}

impl PublishCommand {
    pub fn new(config: Config) -> Box<PublishCommand> {
        Box::new(Self { config })
    }
}

/// Публикует черновик. Технически читает данные из черновика (--draft-post-path) и преобразует данные в
/// grow запись (--post-path) с созданием файлов перевода в зависимости от языка lang.
impl MashinkaCommand for PublishCommand {
    fn run(&self) -> Result<CommandResult, Box<dyn Error>> {
        let draft_path = &self.config.get_draft_path_or_default()?;
        let draft_post = from_grow_draft_file(draft_path)?;

        // Одобряем черновик
        let post = draft_post.approve();

        // todo implement Collection of Detail items
        let mut details: HashMap<String, String> = HashMap::new();

        let command = PUBLISH_COMMAND_NAME.to_string();

        if self.config.is_dry_run() {
            details.insert(String::from("draft_post"), format!("{:#?}", draft_post));
            details.insert(String::from("post"), format!("{:#?}", post));
            return Ok(CommandResult { command, details });
        }

        let (post_path, translation_path) = post.publish(
            &self.config.get_posts_path_or_default(post.lang)?,
            &self.config.get_translation_path_or_default(post.lang)?,
        )?;

        details.insert(String::from("post_path"), format!("{:?}", post_path));
        details.insert(
            String::from("translation_path"),
            format!("{:?}", translation_path),
        );

        Ok(CommandResult { command, details })
    }
}
