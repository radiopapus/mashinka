pub mod draft_post;
pub mod lang;
pub mod post;
pub mod serdes;

pub const DEFAULT_AUTHOR: &str = "Виктор Жарина";
pub const DEFAULT_AUTHOR_EN: &str = "Viktor Zharina";

pub const POST_TEMPLATE: &str = include_str!("grow/templates/post.tpl");
pub const TRANSLATION_TEMPLATE: &str = include_str!("grow/templates/translation.tpl");

pub const ISO8601_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const ISO8601_DATE_FORMAT: &str = "%Y-%m-%d";

const META_DELIMITER: &str = "---";
const KEY_VALUE_DELIMITER: char = ':';
const KEYWORDS_DELIMITER: &str = ",";
const LF: char = '\n';

const MAX_CHARS_IN_DESCRIPTION: usize = 255;
const MAX_CHARS_IN_TITLE: usize = 75;
