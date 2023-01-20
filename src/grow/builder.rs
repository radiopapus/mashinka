use std::path::PathBuf;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::command::Error;
use crate::grow::{
    DESCRIPTION_FIELD_NAME, ISO8601_DATE_TIME_FORMAT, KEYWORDS_FIELD_NAME, MAX_CHARS_IN_DESCRIPTION,
    MAX_CHARS_IN_TITLE, MAX_KEYWORDS_COUNT, TEXT_FIELD_NAME, TITLE_FIELD_NAME,
};
use crate::grow::lang::Lang;
use crate::grow::post::{DraftPost, GrowPost, PostContent};


/// Структура, которая содержит поля, которые будут представлены в каждом Post
/// Это пример для реализации наследования
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct BasePost {
    title: String,
    description: String,
    keywords: Vec<String>,
    lang: Lang,
    text: String
}

#[derive(Default)]
pub struct DraftPostBuilder {
    draft: BasePost
}

impl DraftPostBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait PostBuilder {}
impl PostBuilder for DraftPostBuilder {}
impl PostBuilder for GrowPostBuilder {}

pub trait BasePostBuilder<T, B> where T: PostContent<B>, B: PostBuilder {
    /// ссылка на BasePost
    fn base(&mut self) -> &mut BasePost;

    /// Строит объект на основе полей
    fn build(&self) -> T;


    /// Задает, очищает от пробелов и проверяет корректность заголовка записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если `title` пустой или более `MAX_CHARS_IN_TITLE` символов.
    fn title(&mut self, title: String) -> Result<&mut Self, Error> {
        let title = title.trim();

        if title.is_empty() {
            return Err(Error::EmptyValue(String::from(TITLE_FIELD_NAME)));
        }

        if title.chars().count() >= MAX_CHARS_IN_TITLE {
            return Err(Error::ValueTooLong(
                String::from(TITLE_FIELD_NAME),
                String::from(title),
                MAX_CHARS_IN_TITLE,
            ));
        }

        self.base().title = title.to_string();
        Ok(self)
    }

    /// Задает, очищает от пробелов и проверяет корректность описания записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если `description` пустой или более `MAX_CHARS_IN_DESCRIPTION` символов.
    fn description(&mut self, description: String) -> Result<&mut Self, Error> {
        let description = description.trim();

        if description.is_empty() {
            return Err(Error::EmptyValue(String::from(DESCRIPTION_FIELD_NAME)));
        }

        if description.chars().count() >= MAX_CHARS_IN_DESCRIPTION {
            return Err(Error::ValueTooLong(
                String::from(DESCRIPTION_FIELD_NAME),
                String::from(description),
                MAX_CHARS_IN_DESCRIPTION,
            ));
        }

        self.base().description = description.to_string();
        Ok(self)
    }

    /// Задает, очищает от пробелов и проверяет корректность ключевых слов записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если keywords пустой или более `MAX_KEYWORDS_COUNT` элементов.
    fn keywords(&mut self, keywords: Vec<String>) -> Result<&mut Self, Error> {
        if keywords.is_empty() {
            return Err(Error::EmptyValue(String::from(KEYWORDS_FIELD_NAME)));
        }

        if keywords.len() > MAX_KEYWORDS_COUNT {
            return Err(Error::ValueTooLong(
                String::from(KEYWORDS_FIELD_NAME),
                keywords.join(","),
                MAX_KEYWORDS_COUNT,
            ));
        }

        for k in keywords {
            self.base().keywords.push(k.trim().to_string());
        }

        Ok(self)
    }

    /// Аналогично keywords, но в качестве параметров можно передать строку и указать разделитель.
    /// # Errors
    ///
    /// См. keywords
    fn keywords_as_str(&mut self, keywords: String, delimiter: &str) -> Result<&mut Self, Error> {
        let keywords: Vec<String> = keywords.split(delimiter).map(ToString::to_string).collect();
        self.keywords(keywords)?;
        Ok(self)
    }

    /// Задает язык записи.
    fn lang(&mut self, lang: Lang) -> Result<&mut Self, Error> {
        self.base().lang = lang;
        Ok(self)
    }

    /// Задает текст записи.
    ///
    /// # Errors
    ///
    /// Вернет `Error` если text пустой.
    fn text(&mut self, text: String) -> Result<&mut Self, Error> {
        let text = text.trim();

        if text.is_empty() {
            return Err(Error::EmptyValue(String::from(TEXT_FIELD_NAME)));
        }

        self.base().text = text.to_string();
        Ok(self)
    }
}

impl BasePostBuilder<DraftPost, DraftPostBuilder> for DraftPostBuilder {
    fn base(&mut self) -> &mut BasePost {
        &mut self.draft
    }

    fn build(&self) -> DraftPost {
        // todo add validation for fields here: not empty,
        DraftPost {
            title: self.draft.title.clone(),
            description: self.draft.description.clone(),
            keywords: self.draft.keywords.clone(),
            lang: self.draft.lang,
            text: self.draft.text.clone(),
        }
    }
}

// TODO add check for required fields before build
#[derive(Default)]
pub struct GrowPostBuilder {
    base_post: BasePost,
    author: String,
    image: String,
    slug: String,
    published_at: DateTime<Utc>,
    post_path: PathBuf,
    translation_path: PathBuf,
}

impl GrowPostBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Задает путь абсолютный путь до записи.
    pub fn post_path(&mut self, post_path: PathBuf) -> Result<&mut Self, Error> {
        self.post_path = post_path;
        Ok(self)
    }

    /// Задает путь абсолютный путь до перевода.
    pub fn translation_path(&mut self, translation_path: PathBuf) -> Result<&mut Self, Error> {
        self.translation_path = translation_path;
        Ok(self)
    }

    /// Задает автора.
    pub fn author(&mut self, author: String) -> Result<&mut Self, Error> {
        self.author = author;
        Ok(self)
    }

    /// Задает cover image.
    pub fn image(&mut self, image: String) -> Result<&mut Self, Error> {
        self.image = image;
        Ok(self)
    }

    /// Задает slug.
    pub fn slug(&mut self, slug: String) -> Result<&mut Self, Error> {
        self.slug = slug;
        Ok(self)
    }

    /// Задает published_at .
    pub fn published_at_str(&mut self, published_at: String) -> Result<&mut Self, Error> {
        let dt = NaiveDateTime::parse_from_str(published_at.as_str(), ISO8601_DATE_TIME_FORMAT)
            .map_err(Error::DateTimeError)?;

        self.published_at = DateTime::<Utc>::from_utc(dt, Utc);
        Ok(self)
    }

    /// Задает published_at.
    pub fn published_at(&mut self, published_at: DateTime<Utc>) -> Result<&mut Self, Error> {
        self.published_at = published_at;
        Ok(self)
    }
}

impl BasePostBuilder<GrowPost, GrowPostBuilder> for GrowPostBuilder {
    fn base(&mut self) -> &mut BasePost {
        &mut self.base_post
    }

    fn build(&self) -> GrowPost {
        GrowPost {
            title: self.base_post.title.clone(),
            author: self.author.clone(),
            description: self.base_post.description.clone(),
            keywords: self.base_post.keywords.clone(),
            lang: self.base_post.lang,
            published_at: self.published_at,
            slug: self.slug.clone(),
            text: self.base_post.text.clone(),
        }
    }
}
