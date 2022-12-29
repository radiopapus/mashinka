pub mod commands;
pub mod config;
pub mod grow;

pub const TEST_DRY_RUN_ARG_KEY: &str = "--dry-run";
pub const TEST_DRAFT_PATH_ARG_KEY: &str = "--draft-path";
pub const TEST_POSTS_PATH_ARG_KEY: &str = "--posts-path";
pub const TEST_TRANSLATIONS_PATH_ARG_KEY: &str = "--translations-path";

pub const TEST_TMP_TRANSLATION_FILE_NAME: &str = "translation.po";
pub const TEST_TMP_DRAFT_FILE_NAME: &str = "draft.md";
pub const TEST_EMPTY_CONTENT: &str = "";
