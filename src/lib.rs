pub mod commands;
pub mod grow;
pub mod config;

pub const BIN_NAME: &str = "mashinka";

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

pub const TEST_POST_TITLE: &str = "Это тестовый заголовок";
pub const TEST_AUTHOR: &str = "Виктор Жарина";
pub const TEST_DESCRIPTION: &str = "Тестовое описание для записи";
pub const TEST_SLUG: &str = "eto-testovyi-zagolovok";
pub const TEST_IMAGE: &str = "/static/images/default.png";
pub const TEST_KEYWORDS_AS_STRING: &str = "бумага,А4,297 мм";
pub const TEST_LANG_AS_STRING: &str = "ru";
pub const TEST_CONTENT: &str = "test_content";

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

pub const TEST_TRANSLATION_TEMPLATE: &str = r#"msgid "[id]"
msgstr "[value]""#;

pub const TEST_DRY_RUN_ARG_KEY: &str = "--dry-run";
pub const TEST_DRAFT_PATH_ARG_KEY: &str = "--draft-path";
pub const TEST_POSTS_PATH_ARG_KEY: &str = "--posts-path";
pub const TEST_TRANSLATIONS_PATH_ARG_KEY: &str = "--translations-path";

pub const TEST_TMP_TRANSLATION_FILE_NAME: &str = "translation.po";
pub const TEST_TMP_DRAFT_FILE_NAME: &str = "draft.md";
pub const TEST_EMPTY_CONTENT: &str = "";