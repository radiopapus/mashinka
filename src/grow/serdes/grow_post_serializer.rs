use crate::grow::post::Post;
use crate::grow::serdes::process_template;
use crate::grow::{ISO8601_DATE_TIME_FORMAT, KEYWORDS_DELIMITER};
use std::collections::HashMap;

/// Преобразует Post в форматированную строку для grow записи.
/// title здесь это идентификатор - slug, который будет использован для системы перевода
pub fn serialize_with_template(post: &Post, template: String) -> String {
    let published_date = post
        .published_date_time
        .format(ISO8601_DATE_TIME_FORMAT)
        .to_string();

    let template_tuple = [
        ("title", post.slug.clone()),
        ("author", post.author.clone()),
        ("description", post.description.clone()),
        ("image", "/static/images/default.png".to_string()),
        ("lang", post.lang.to_string()),
        ("slug", post.slug.clone()),
        ("content", post.draft_content.clone()),
        ("publish_date", published_date),
        ("keywords", post.keywords.join(KEYWORDS_DELIMITER)),
    ];

    let key_values = HashMap::from(template_tuple.map(|(k, v)| (k, v)));

    process_template(template, key_values)
}
