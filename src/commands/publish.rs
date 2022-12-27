use crate::commands::{CommandResult, MashinkaCommand, PUBLISH_COMMAND_NAME};
use crate::config::Config;
use crate::grow::serdes::grow_draft_deserializer::from_grow_draft_file;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct PublishCommand {
    config: Config,
}

impl PublishCommand {
    pub fn new(config: Config) -> Box<PublishCommand> {
        Box::new(Self { config })
    }
}

struct Details<'a> {
    post_path: &'a Path,
    translation_path: &'a Path,
}

impl Display for Details<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Публикует черновик. Технически читает данные из черновика (--draft-post-path) и преобразует данные в
/// grow запись (--post-path) с созданием файлов перевода в зависимости от языка lang.
impl MashinkaCommand for PublishCommand {
    fn run(&self) -> Result<CommandResult, Box<dyn Error>> {
        let draft_post = from_grow_draft_file(&self.config.get_draft_path_or_default());

        // Одобряем черновик
        let post = draft_post.approve();

        let (posts_path, translation_path) = (
            self.config.get_posts_path_or_default(post.lang),
            self.config.get_translation_path_or_default(post.lang),
        );

        let post_file_name = &post.build_file_name(&posts_path);

        let abs_post_path = PathBuf::from(&posts_path.as_os_str()).join(&post_file_name);
        let abs_translation_path = PathBuf::from(&translation_path.as_os_str());

        if self.config.is_dry_run() {
            return Ok(CommandResult {
                command: PUBLISH_COMMAND_NAME.to_string(),
                details: format!(
                    "{:#?}, {:#?}, post_path: {:?}, translation_path: {:?}",
                    draft_post, post, abs_post_path, abs_translation_path
                ),
            });
        }

        // Запись и перевод
        let write_files = || -> std::io::Result<()> {
            fs::write(post_file_name, &post.build_content())?;

            File::options()
                .append(true)
                .open(&translation_path)?
                .write_all(&post.build_translation().as_bytes())?;

            Ok(())
        };

        if let Err(err) = write_files() {
            return Err(Box::new(err));
        }

        Ok(CommandResult {
            command: PUBLISH_COMMAND_NAME.to_string(),
            details: format!(
                r#"
            post_path: {:?},
            translation_path: {:?}"#,
                abs_post_path, abs_translation_path
            ),
        })
    }
}
