mod post;
mod draft_post;
mod lang;

const ISO8601_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const ISO8601_DATE_FORMAT: &str = "%Y-%m-%d";

const META_DELIMITER: &str = "---";
const KEY_VALUE_DELIMITER: char = ':';
const KEYWORDS_DELIMITER: char = ',';
const LF: char = '\n';

const MAX_CHARS_IN_DESCRIPTION: usize = 255;
const MAX_CHARS_IN_TITLE: usize = 75;
