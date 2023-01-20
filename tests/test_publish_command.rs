mod common;

#[cfg(test)]
pub mod test_publish_command {
    use assert_fs::prelude::{FileWriteStr};
    use assert_fs::{NamedTempFile, TempDir};
    use chrono::Utc;
    use std::{fs};
    use std::path::Path;
    use assert_fs::fixture::{PathChild, PathCreateDir};

    use crate::common::{BIN_NAME, TEST_DRAFT_PATH_ARG_KEY, TEST_DRY_RUN_ARG_KEY, TEST_POSTS_PATH_ARG_KEY,
        TEST_TMP_DRAFT_FILE_NAME, TEST_TRANSLATIONS_PATH_ARG_KEY};

    use mashinka::command::PUBLISH_COMMAND_NAME;
    use mashinka::grow::lang::Lang;
    use mashinka::grow::{ISO8601_DATE_FORMAT};
    use mashinka::grow::post::{DraftPost, GrowPostTranslation};

    pub const TEST_DRAFT_CONTENT: &str = r#"---
title: Это тестовый заголовок
lang: ru
description: Тестовое описание для записи
keywords: бумага,А4,297 мм
---

test_text
"#;

    struct FixturedData {
        pub base_dir: TempDir,
        pub draft_path: NamedTempFile,
    }

    fn init() -> FixturedData {
        let tmp_dir = TempDir::new().expect("Can't create tmp dir for posts.");
        tmp_dir.child("posts/ru").create_dir_all().unwrap();
        tmp_dir.child("posts/en").create_dir_all().unwrap();

        let draft_path = NamedTempFile::new(TEST_TMP_DRAFT_FILE_NAME)
            .expect("Can't create tmp draft file.");

        draft_path.write_str(TEST_DRAFT_CONTENT)
            .expect("Can't write to tmp draft file.");

        tmp_dir.child("translations/ru/LC_MESSAGES").create_dir_all().unwrap();
        tmp_dir.child("translations/en/LC_MESSAGES").create_dir_all().unwrap();

        FixturedData {
            base_dir: tmp_dir,
            draft_path,
        }
    }

    // Из черновика будет создан post (чистовик) с переводом согласно формату.
    // в этом тесте проверяем то, что данные из черновика будут записаны в чистовик
    // будет создан перевод для чистовика согласно заданному языку
    #[test]
    fn test_run_publish_command() {
        let test_data = init();

        let posts_path = test_data.base_dir.path().join("posts");
        let posts_path = posts_path.to_str().unwrap();
        let draft_path = test_data.draft_path.path().to_str().unwrap();
        let translation_path = test_data.base_dir.path().join("translations");
        let translation_path = translation_path.to_str().unwrap();

        let output = test_bin::get_test_bin(BIN_NAME)
            .arg(PUBLISH_COMMAND_NAME)
            .args([
                format!("{}={}", TEST_DRAFT_PATH_ARG_KEY, draft_path),
                format!("{}={}", TEST_POSTS_PATH_ARG_KEY, posts_path),
                format!("{}={}", TEST_TRANSLATIONS_PATH_ARG_KEY, translation_path),
            ])
            .output();

        dbg!(&output);

        assert!(output.unwrap().status.success());
        assert!(test_data.draft_path.exists());

        let now = Utc::now();
        let formatted_date = now.format(ISO8601_DATE_FORMAT).to_string();

        // 2022-12-24-eto-testoviy-zagolovok@ru.md
        let expected_post_file_name = format!("{formatted_date}-eto-testoviy-zagolovok@ru.md");

        let post_file_path = format!("{}/{}/{}", posts_path, "ru", expected_post_file_name);
        let expected_post_file = Path::new(&post_file_path);
        dbg!(expected_post_file);

        assert!(expected_post_file.exists());

        // then check post content
        let post_file_content = fs::read_to_string(expected_post_file).unwrap();

        let expected_draft_post = DraftPost {
            title: "Это тестовый заголовок".to_string(),
            description: "Тестовое описание для записи".to_string(),
            keywords: vec!["бумага".to_string(), "А4".to_string(), "297 мм".to_string()],
            lang: Lang::Ru,
            text: "test_text".to_string(),
        };

        let approved = expected_draft_post.approve();
        let grow_post = approved.to_grow_post().unwrap();

        assert_eq!(grow_post.to_string(), post_file_content);

        // then check translations
        dbg!(&translation_path);
        let translation_file_content = fs::read_to_string(test_data.base_dir.path().join("translations/ru/LC_MESSAGES/messages.po")).unwrap();

        let translation = GrowPostTranslation {
            id: grow_post.slug,
            translated_value: grow_post.title,
        };
        assert_eq!(translation.to_string(), translation_file_content);
    }

    // В отличие от предыдущего теста данные не будут записаны в файлы, но результат работы команды
    // будет выведен в std::output.
    #[test]
    fn test_run_publish_command_dry_run() {
        let test_data = init();

        let posts_path = test_data.base_dir.path().join("posts");
        let posts_path = posts_path.to_str().unwrap();
        let draft_path = test_data.draft_path.path().to_str().unwrap();
        let translation_path = test_data.base_dir.path().join("translations");
        let translation_path = translation_path.to_str().unwrap();

        let output = test_bin::get_test_bin(BIN_NAME)
            .arg(PUBLISH_COMMAND_NAME)
            .arg(TEST_DRY_RUN_ARG_KEY)
            .args([
                format!("{}={}", TEST_DRAFT_PATH_ARG_KEY, draft_path),
                format!("{}={}", TEST_POSTS_PATH_ARG_KEY, posts_path),
                format!( "{}={}", TEST_TRANSLATIONS_PATH_ARG_KEY, translation_path),
            ])
            .output();

        dbg!(&output);

        let output = output.unwrap();
        assert!(output.status.success());

        let now = Utc::now();
        let formatted_date = now.format(ISO8601_DATE_FORMAT).to_string();

        // 2022-12-24-eto-testoviy-zagolovok@ru.md
        let expected_post_file_name = format!("{formatted_date}-eto-testoviy-zagolovok@ru.md");

        let post_file_path = format!("{}/{}/{}", posts_path, "ru", expected_post_file_name);
        let expected_post_file = Path::new(&post_file_path);

        assert!(!expected_post_file.exists());

        let translation_file = test_data.base_dir.path().join("translations/ru/LC_MESSAGES/messages.po");
        let translation_file = Path::new(&translation_file);

        assert!(!translation_file.exists());
    }
}

