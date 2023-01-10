#![allow(dead_code)]

pub mod lang;
pub mod post;
pub mod serdes;
pub mod builder;

pub const TITLE_FIELD_NAME: &str = "title";
pub const DESCRIPTION_FIELD_NAME: &str = "description";
pub const TEXT_FIELD_NAME: &str = "text";
pub const LANGUAGE_FIELD_NAME: &str = "lang";
pub const KEYWORDS_FIELD_NAME: &str = "keywords";
pub const AUTHOR_FIELD_NAME: &str = "author";
pub const IMAGE_FIELD_NAME: &str = "image";
pub const SLUG_FIELD_NAME: &str = "slug";
pub const PUBLISHED_DATE_FIELD_NAME: &str = "published";

pub const DEFAULT_AUTHOR: &str = "Виктор Жарина";
pub const DEFAULT_AUTHOR_EN: &str = "Viktor Zharina";

pub const POST_TEMPLATE: &str = include_str!("grow/templates/post.tpl");
pub const TRANSLATION_TEMPLATE: &str = include_str!("grow/templates/translation.tpl");
pub const DRAFT_TEMPLATE: &str = include_str!("grow/templates/draft_post.tpl");

pub const ISO8601_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const ISO8601_DATE_FORMAT: &str = "%Y-%m-%d";

const META_DELIMITER: &str = "---";
const KEY_VALUE_DELIMITER: &str = ":";
const KEYWORDS_DELIMITER: &str = ",";
const LF: char = '\n';

const MAX_CHARS_IN_DESCRIPTION: usize = 255;
const MAX_CHARS_IN_TITLE: usize = 75;
const MAX_KEYWORDS_COUNT: usize = 10;
