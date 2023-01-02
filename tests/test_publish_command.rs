mod common;

#[cfg(test)]
pub mod test_publish_command {
    use assert_fs::prelude::{FileTouch, FileWriteStr};
    use assert_fs::{NamedTempFile, TempDir};
    use chrono::Utc;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    use crate::common::BIN_NAME;
    use mashinka::command::PUBLISH_COMMAND_NAME;
    use mashinka::grow::lang::Lang;
    use mashinka::grow::serdes::process_template;
    use mashinka::grow::{
        ISO8601_DATE_FORMAT, ISO8601_DATE_TIME_FORMAT, TEST_AUTHOR, TEST_CONTENT, TEST_DESCRIPTION,
        TEST_DRAFT_CONTENT, TEST_IMAGE, TEST_KEYWORDS_AS_STRING, TEST_LANG_AS_STRING,
        TEST_POST_CONTENT_TEMPLATE, TEST_POST_TITLE, TEST_SLUG, TEST_TRANSLATION_TEMPLATE,
    };
    use mashinka::{
        TEST_DRAFT_PATH_ARG_KEY, TEST_DRY_RUN_ARG_KEY, TEST_EMPTY_CONTENT, TEST_POSTS_PATH_ARG_KEY,
        TEST_TMP_DRAFT_FILE_NAME, TEST_TMP_TRANSLATION_FILE_NAME, TEST_TRANSLATIONS_PATH_ARG_KEY,
    };

    struct FixturedData {
        pub posts_path: TempDir,
        pub translation_path: NamedTempFile,
        pub draft_path: NamedTempFile,
    }

    fn init() -> FixturedData {
        let posts_path = TempDir::new().expect("Can't create tmp dir for posts.");

        let draft_path =
            NamedTempFile::new(TEST_TMP_DRAFT_FILE_NAME).expect("Can't create tmp draft file.");

        draft_path
            .write_str(TEST_DRAFT_CONTENT)
            .expect("Can't write to tmp draft file.");

        let translation_path = NamedTempFile::new(TEST_TMP_TRANSLATION_FILE_NAME).unwrap();

        translation_path
            .touch()
            .expect("Can't touch translation file.");

        FixturedData {
            posts_path,
            translation_path,
            draft_path,
        }
    }

    // Из черновика будет создан post (чистовик) с переводом согласно формату.
    // в этом тесте проверяем то, что данные из черновика будут записаны в чистовик
    // будет создан перевод для чистовика соглас заданному языку
    #[test]
    fn test_run_publish_command() {
        let test_data = init();

        let posts_path = test_data.posts_path.path().to_str().unwrap();

        let output = test_bin::get_test_bin(BIN_NAME)
            .arg(PUBLISH_COMMAND_NAME)
            .args([
                format!(
                    "{}={}",
                    TEST_DRAFT_PATH_ARG_KEY,
                    test_data.draft_path.path().to_str().unwrap()
                ),
                format!("{}={}", TEST_POSTS_PATH_ARG_KEY, posts_path),
                format!(
                    "{}={}",
                    TEST_TRANSLATIONS_PATH_ARG_KEY,
                    test_data.translation_path.path().to_str().unwrap()
                ),
            ])
            .output();

        dbg!(&output);

        assert!(output.unwrap().status.success());
        assert!(test_data.draft_path.exists());

        let now = Utc::now();
        let formatted_date = now.format(ISO8601_DATE_FORMAT).to_string();
        let formatted_date_time = now.format(ISO8601_DATE_TIME_FORMAT).to_string();

        // 2022-12-24-eto-testovyi-zagolovok@ru.md
        let expected_post_file_name =
            format!("{formatted_date}-{}@{}.md", TEST_SLUG, TEST_LANG_AS_STRING);

        let post_file_path = format!("{}/{}", posts_path, expected_post_file_name);
        let expected_post_file = Path::new(&post_file_path);

        assert!(expected_post_file.exists());

        // then check post content
        let post_file_content = fs::read_to_string(expected_post_file).unwrap();

        let post_template_vars = HashMap::from(
            [
                ("title", TEST_POST_TITLE),
                ("author", TEST_AUTHOR),
                ("description", TEST_DESCRIPTION),
                ("image", TEST_IMAGE),
                ("keywords", TEST_KEYWORDS_AS_STRING),
                ("slug", TEST_SLUG),
                ("content", TEST_CONTENT),
                ("lang", &Lang::Ru.to_string()),
                ("publish_date", &formatted_date_time),
            ]
            .map(|(k, v)| (k, v.to_string())),
        );

        let expected_post_content =
            process_template(TEST_POST_CONTENT_TEMPLATE.to_string(), post_template_vars);

        assert_eq!(expected_post_content, post_file_content);

        // then check translations
        let translation_file_content = fs::read_to_string(test_data.translation_path).unwrap();

        let translation_template_vars = HashMap::from(
            [("id", TEST_SLUG), ("value", TEST_POST_TITLE)].map(|(k, v)| (k, v.to_string())),
        );

        let expected_translation_content = process_template(
            TEST_TRANSLATION_TEMPLATE.to_string(),
            translation_template_vars,
        );

        assert_eq!(expected_translation_content, translation_file_content);
    }

    // В отличие от предыдущего теста данные не будут записаны в файлы, но результат работы команды
    // будет выведен в std::output.
    #[test]
    fn test_run_publish_command_dry_run() {
        let test_data = init();

        let posts_path = test_data.posts_path.path().to_str().unwrap();

        let output = test_bin::get_test_bin(BIN_NAME)
            .arg(PUBLISH_COMMAND_NAME)
            .arg(TEST_DRY_RUN_ARG_KEY)
            .args([
                format!(
                    "{}={}",
                    TEST_DRAFT_PATH_ARG_KEY,
                    test_data.draft_path.path().to_str().unwrap()
                ),
                format!("{}={}", TEST_POSTS_PATH_ARG_KEY, posts_path),
                format!(
                    "{}={}",
                    TEST_TRANSLATIONS_PATH_ARG_KEY,
                    test_data.translation_path.path().to_str().unwrap()
                ),
            ])
            .output();

        dbg!(&output);

        let output = output.unwrap();
        assert!(output.status.success());

        let now = Utc::now();
        let formatted_date = now.format(ISO8601_DATE_FORMAT).to_string();

        // 2022-12-24-eto-testovyi-zagolovok@ru.md
        let expected_post_file_name =
            format!("{formatted_date}-{}@{}.md", TEST_SLUG, TEST_LANG_AS_STRING);

        let post_file_path = format!("{}/{}", posts_path, expected_post_file_name);
        let expected_post_file = Path::new(&post_file_path);

        assert!(!expected_post_file.exists());

        let translation_path = test_data.translation_path.path().to_str().unwrap();
        let translation_content = fs::read_to_string(Path::new(translation_path)).unwrap();

        assert_eq!(TEST_EMPTY_CONTENT, translation_content);
    }
}
