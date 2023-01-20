use std::fs;
use crate::command::{Command, CommandResult, Details, Error, PUBLISH_COMMAND_NAME};
use crate::config::Config;
use crate::grow::post::{DraftPost, GrowPostTranslation, WriterWrapper};
use crate::grow::serdes::GrowDeserializer;

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
        let config = &self.config;
        let draft_path = &config.get_draft_path_or_default()?;
        let draft_file_content = fs::read_to_string(draft_path).map_err(Error::ReadFile)?;
        let draft_post = DraftPost::deserialize(draft_file_content.as_str())?;

        let command = PUBLISH_COMMAND_NAME.to_string();

        let mut details = Details::new();

        let posts_path = config.get_posts_path_or_default()?;
        // Одобряем черновик
        let grow_post = draft_post.to_grow_post()?;
        let grow_post_path = grow_post.build_post_path(&posts_path);
        details.push("post_path".to_string(), format!("{:#?}", grow_post_path));
        // перевод
        let translation = GrowPostTranslation { id: grow_post.slug.clone(), translated_value: grow_post.title.clone() };
        let translation_path = config.get_translations_path_or_default()?
            .join(grow_post.lang.to_lowercase())
            .join("LC_MESSAGES/messages.po");

        details.push("translation_path".to_string(), format!("{:#?}", translation_path));

        if config.is_dry_run() {
            details.push("draft_post".to_string(), format!("{:#?}", draft_post));
            details.push("post".to_string(), format!("{:#?}", grow_post));
            return Ok(CommandResult { command, details });
        }

        let write_in_transaction = || -> Result<(), Error> {
            WriterWrapper::write_file(&grow_post_path, &grow_post.to_string())?;
            WriterWrapper::write_file_with_append(&translation_path, &translation.to_string())?;
            Ok(())
        };

        write_in_transaction()?;

        Ok(CommandResult { command, details })
    }
}
