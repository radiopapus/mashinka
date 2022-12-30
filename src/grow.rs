#![allow(dead_code)]

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
const MAX_KEYWORDS_COUNT: usize = 75;

// tests
pub const TEST_POST_TITLE: &str = "Это тестовый заголовок";
pub const TEST_AUTHOR: &str = "Виктор Жарина";
pub const TEST_DESCRIPTION: &str = "Тестовое описание для записи";
pub const TEST_SLUG: &str = "eto-testovyi-zagolovok";
pub const TEST_IMAGE: &str = "/static/images/default.png";
pub const TEST_KEYWORDS_AS_STRING: &str = "бумага,А4,297 мм";
pub const TEST_LANG_AS_STRING: &str = "ru";
pub const TEST_CONTENT: &str = "test_content";

pub const TEST_DRAFT_TITLE: &str = "Это тестовый заголовок";
pub const TEST_DRAFT_TEMPLATE: &str = r#"
title: [title]
lang: [lang]
description: [description]
keywords: [keywords]
---

[content]
"#;

pub const TEST_DRAFT_CONTENT: &str = r#"
title: Это тестовый заголовок
lang: ru
description: Тестовое описание для записи
keywords: бумага,А4,297 мм
---

test_content
"#;

pub const TEST_POST_CONTENT_TEMPLATE: &str = r#"---
$title@: [slug]
author@: [author]
description: [description]
keywords: [keywords]
image: [image]
slug[lang]: [slug]
$dates:
  published: [publish_date]
---
[content]"#;

pub const TEST_TRANSLATION_TEMPLATE: &str = r#"

msgid "[id]"
msgstr "[value]"

"#;
