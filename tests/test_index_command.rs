mod common;

#[cfg(test)]
pub mod test_publish_command {
    use std::collections::HashMap;
    use std::{fs};
    use assert_fs::prelude::{FileTouch, FileWriteStr, PathChild, PathCreateDir};
    use assert_fs::{TempDir};
    use std::path::{PathBuf};
    use std::str::from_utf8;

    use crate::common::{BIN_NAME, TEST_DRY_RUN_ARG_KEY, TEST_INDEX_PATH_ARG_KEY, TEST_POSTS_PATH_ARG_KEY, TEST_TMP_INDEX_FILE_NAME, TEST_TMP_TRANSLATION_FILE_NAME, TEST_TRANSLATIONS_PATH_ARG_KEY};

    use mashinka::command::{INDEX_COMMAND_NAME};

    pub const TEST_TMP_POST_FILE_NAME_RU: &str = "2021-11-16-19-15-19-test-post@ru.md";
    pub const TEST_TMP_POST_FILE_NAME_EN: &str = "2021-11-16-19-15-19-test-post-another@en.md";

    pub const TEST_POST_CONTENT_RU: &str = r#"---
$title@: title
author@: автор
description: описание
keywords: слово, другое
$order: 332
image: /static/images/default.png
slugEn: slug-en
$dates:
  published: 2021-11-16 19:16:00
---

<h3>тест</h3>
"#;

    const TEST_POST_CONTENT_EN: &str = r#"---
$title@: title-another
author@: author
description: description
keywords: k1,k2
$order: 331
image: /static/images/default.png
slugEn: title
$dates:
  published: 2021-11-16 19:15:19
---

<h3>text</h3>
"#;

    pub const TEST_TRANSLATION_CONTENT_RU: &str = r#"msgid "title"
msgstr "перевод"
"#;

    pub const TEST_TRANSLATION_CONTENT_EN: &str = r#"msgid "title-another"
msgstr "translation"
"#;

    struct FixturedData {
        pub posts_path: PathBuf,
        pub translations_path: PathBuf,
        pub index_path: PathBuf,
    }

    fn init(persist: bool) -> FixturedData {
        let tmp = TempDir::new().expect("Can't create tmp dir for posts.").into_persistent_if(persist);
        let available_languages = &["ru", "en"];

        // todo refactor posts like translations
        let posts_path = tmp.child("posts");
        posts_path.create_dir_all()
            .expect("Can't create tmp dir for ru translations.");

        let posts_map = HashMap::from([
            ("ru", (TEST_TMP_POST_FILE_NAME_RU, TEST_POST_CONTENT_RU)),
            ("en", (TEST_TMP_POST_FILE_NAME_EN, TEST_POST_CONTENT_EN))
        ]);

        let translations_path = tmp.child("translations");
        translations_path.create_dir_all()
            .expect("Can't create tmp dir for translations.");

        let translation_map = HashMap::from([
            ("ru", TEST_TRANSLATION_CONTENT_RU),
            ("en", TEST_TRANSLATION_CONTENT_EN)
        ]);

        for lang in available_languages {
            let posts_path = posts_path.child(lang);
            posts_path.create_dir_all()
                .expect("Can't create tmp dir for post.");

            posts_path.child(posts_map[lang].0)
                .write_str(posts_map[lang].1)
                .expect("Can't create tmp post file.");

            let translations_path = translations_path.child(format!("{lang}/LC_MESSAGES"));
            translations_path.create_dir_all()
                .expect("Can't create tmp dir for translations.");

            translations_path.child(TEST_TMP_TRANSLATION_FILE_NAME)
                .write_str(translation_map[lang])
                .expect("Can't create tmp translation file.");
        }

        let index_path = tmp.child("index");
        index_path.create_dir_all().expect("Can't create dir for index.");

        let index_file = index_path.child(TEST_TMP_INDEX_FILE_NAME);
        index_file.touch().expect("Can't create tmp data json file.");

        FixturedData {
            posts_path: posts_path.to_path_buf(),
            translations_path: translations_path.to_path_buf(),
            index_path: index_file.to_path_buf()
        }
    }

    #[test]
    fn test_run_index_command() {
        let test_data = init(true); //todo why true? can make it false, what is going on here?

        let index_path = test_data.index_path.to_str().unwrap();
        let posts_path = test_data.posts_path.to_str().unwrap();
        let translations_path = test_data.translations_path.to_str().unwrap();

        let args = [
            format!("{}={}", TEST_INDEX_PATH_ARG_KEY, index_path),
            format!("{}={}", TEST_POSTS_PATH_ARG_KEY, posts_path),
            format!("{}={}", TEST_TRANSLATIONS_PATH_ARG_KEY, translations_path)
        ];

        let output = test_bin::get_test_bin(BIN_NAME)
            .arg(INDEX_COMMAND_NAME)
            .args(args)
            .output();

        dbg!(&output);

        let output = output.unwrap();
        assert!(output.status.success());
        let data_json = fs::read_to_string(test_data.index_path).unwrap();
        let expected_index_data = r#"[{ru_part},{en_part}]"#
            .replace("{ru_part}", r#"{"id": "/ru/posts/title", "title": "перевод", "content": "тест"}"#)
            .replace("{en_part}", r#"{"id": "/en/posts/title-another", "title": "translation", "content": "text"}"#);
        assert_eq!(expected_index_data, data_json);
    }

    // В отличие от предыдущего теста данные не будут записаны в файлы, но результат работы команды
    // будет выведен в std::output.
    #[test]
    fn test_run_index_command_dry_run() {
        let test_data = init(true);

        let index_path = test_data.index_path.to_str().unwrap();
        let posts_path = test_data.posts_path.to_str().unwrap();
        let translations_path = test_data.translations_path.to_str().unwrap();

        let args = [
            format!("{}={}", TEST_INDEX_PATH_ARG_KEY, index_path),
            format!("{}={}", TEST_POSTS_PATH_ARG_KEY, posts_path),
            format!("{}={}", TEST_TRANSLATIONS_PATH_ARG_KEY, translations_path)
        ];

        let output = test_bin::get_test_bin(BIN_NAME)
            .arg(INDEX_COMMAND_NAME)
            .arg(TEST_DRY_RUN_ARG_KEY)
            .args(args)
            .output();

        dbg!(&output);

        let output = output.unwrap();
        let as_string = output.stdout.to_ascii_lowercase();
        let stdout = from_utf8(&as_string).unwrap();
        assert!(output.status.success());
        assert!(stdout.contains("index_data"));
        assert!(stdout.contains("/ru/posts/title"));
        assert!(stdout.contains("перевод"));
        assert!(stdout.contains("/en/posts/title-another"));
        assert!(stdout.contains("translation"));
    }
}
