use crate::command::{Command, CommandResult, Error, PUBLISH_COMMAND_NAME};
use crate::config::Config;
use crate::grow::draft_post::DraftPost;
use std::collections::HashMap;

pub struct Publish {
    config: Config,
}

impl Publish {
    pub fn new(config: Config) -> Box<Publish> {
        Box::new(Self { config })
    }
}

/// Публикует черновик. Технически читает данные из черновика (--draft-post-path) и преобразует данные в
/// grow запись (--post-path) с созданием файлов перевода в зависимости от языка lang.
impl Command for Publish {
    fn run(&self) -> Result<CommandResult, Error> {
        let draft_path = self.config.get_draft_path_or_default()?;
        let draft_post = DraftPost::from_grow_draft_file(&draft_path)?;

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
