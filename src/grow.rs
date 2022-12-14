mod post;
mod draft_post;
mod lang;

const DEFAULT_AUTHOR: &str = "Viktor Zharina";
const ISO8601_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const ISO8601_DATE_FORMAT: &str = "%Y-%m-%d";

const META_DELIMITER: &str = "---";
const KEY_VALUE_DELIMITER: char = ':';
const KEYWORDS_DELIMITER: char = ',';
const LF: char = '\n';

const MAX_CHARS_IN_DESCRIPTION: usize = 255;
const MAX_CHARS_IN_TITLE: usize = 75;

// test
const TEST_DRAFT_TITLE: &str = "Перевод - Почему бумага формата А4 имеет размер 297 мм на 210 мм?";

const TEST_DESCRIPTION: &str = "Тестовое описание для записи";
const TEST_SLUG: &str = "perevod-pochemu-bumaga-formata-a4-imeet-razmer-297-mm-na-210-mm";

// Заголовок записи это идентификатор для переводчика, он не равен TEST_DRAFT_TITLE, но равен slug
const TEST_POST_TITLE: &str = TEST_SLUG;

const TEST_KEYWORDS_AS_STRING: &str = "бумага,А4,297 мм";
const TEST_LANG_AS_STRING: &str = "ru";
const TEST_CONTENT: &str = "_Вкратце: размер листа А0 равен 1 189 мм на 841 мм (1 м<sup>2</sup>). Площадь 1 м<sup>2</sup>
скорее всего выбрана из-за удобства измерения и расчетов. Сотношение сторон примерно равно sqrt2 (1.41) и
выбрано не случайно. Это дает возможность получать листы меньшего размера, сохраняя соотношение сторон.
Таким образом, <i>чтобы получить из листа формата</i> А0 лист формата А4 нужно свернуть лист 4 раза.
 Вот и получается 1 189 / 4 = 297.25, что примерно равно 297 мм._";