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
        let draft_file_content = fs::read_to_string(config.get_draft_path_or_default()?).map_err(
            Error::ReadFile
        )?;
        let draft_post = DraftPost::deserialize(draft_file_content.as_str())?;

        // Одобряем черновик
        let approved_post = draft_post.approve();

        let command = PUBLISH_COMMAND_NAME.to_string();

        let mut details = Details::new();

        // grow запись
        let posts_path = config.get_posts_path_or_default(approved_post.lang)?;
        let grow_post = approved_post.to_grow_post()?;
        let grow_post_path = grow_post.build_post_path(&posts_path);
        details.push("post_path".to_string(), format!("{:#?}", grow_post_path));

        // перевод
        let translation = GrowPostTranslation { lang: grow_post.lang, id: grow_post.slug.clone(), translated_value: grow_post.title.clone() };
        let translation_path = config.get_translation_path_or_default(approved_post.lang)?;
        details.push("translation_path".to_string(), format!("{:#?}", translation_path));

        if config.is_dry_run() {
            details.push("draft_post".to_string(), format!("{:#?}", draft_post));
            details.push("post".to_string(), format!("{:#?}", approved_post));
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
