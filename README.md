```
 ███    ███  █████  ███████ ██   ██ ██ ███    ██ ██   ██  █████  
 ████  ████ ██   ██ ██      ██   ██ ██ ████   ██ ██  ██  ██   ██
 ██ ████ ██ ███████ ███████ ███████ ██ ██ ██  ██ █████   ███████
 ██  ██  ██ ██   ██      ██ ██   ██ ██ ██  ██ ██ ██  ██  ██   ██
 ██      ██ ██   ██ ███████ ██   ██ ██ ██   ████ ██   ██ ██   ██
```                                                             
<a alt="twitch" href="twitch.tv/radiopapus"><img src="https://upload.wikimedia.org/wikipedia/commons/3/3a/Twitch_mit_Glitch.png" alt="twitch" height="32"/></a>
[![telegram](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5a/Telegram_2019_simple_logo.svg/32px-Telegram_2019_simple_logo.svg.png)](https://t.me/radiopapus)
<a alt="youtube" href="https://www.youtube.com/@radiopapus"><img src="https://upload.wikimedia.org/wikipedia/commons/9/98/YouTube_Play_Button_Free.png" alt="youtube" height="32"/></a>
<a alt="VK" href="https://vk.com/radiopapus"><img src="https://upload.wikimedia.org/wikipedia/commons/4/47/V_Kontakte_Russian_V.png" alt="VK" height="32"/></a>

# Предыстория
Когда давным-давно у меня был блог на Wordpress. Потом я узнал про статический генератор
сайтов grow (grow.io). Я решил перенести данные в grow и написал экспортер. Часть данных сломал, а часть потерял, 
но большая часть была перенесена и выгружена в облачное хранилище в виде набора html страниц.
  
# Mashinka

Это экспериментальный проект, целью которого является изучение языка программирования Rust.
Он был создан в свободное время. В давние времена я играл в gta2 и мне запомнился [этот момент](https://www.youtube.com/watch?v=poTdkwoPxiI). 
Отсюда и название.

При создании записей нужно проделать ряд операций: создать файл с названием записи
(дата-название-язык), в файл написать контент в определенном формате, создать индекс для поиска,
сгенерить контент, который выгрузить в облачное хранилище. Хотелось упростить эту работу.

У меня был набор PHP-скриптов, который делал эту работу и я решил переписать их на Rust и оценить трудоемкость 
такой работы.

```mashinka help``` - выведет информацию о командах, которые поддерживает
утилита.

#### Почему Rust, а не {ваш язык программирования}, почему ubuntu и т.д.
[Почему](https://www.youtube.com/watch?v=vC3jnJy_Ids&t=59s)

## Полезные ссылки
[Rust Programming Language](https://doc.rust-lang.org/book)

[Rust Language Cheat Sheet](https://cheats.rs/)

[Naming - Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html)

[Conventions for Command Line Options](https://nullprogram.com/blog/2020/08/01/)

[Testing - Command Line Applications in Rust](https://rust-cli.github.io/book/tutorial/testing.html)

[Paste.rs](https://paste.rs/web)

[Panic messages for humans](https://github.com/rust-cli/human-panic)

[String-conversions](https://profpatsch.de/notes/rust-string-conversions)

[Some summaries on Rust string literals](https://www.sobyte.net/post/2022-07/rust-string/)

[Comprehensive Rust](https://google.github.io/comprehensive-rust/welcome-day-1.html)

[Small exercises to get you used to reading and writing Rust code!](https://github.com/rust-lang/rustlings)

[Effective Rust](https://www.lurklurk.org/effective-rust/iterators.html)

[Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)

[The Little Book of Rust Macros](https://veykril.github.io/tlborm/)

[Writing Non-Trivial Macros in Rust · Michael-F-Bryan](https://adventures.michaelfbryan.com/posts/non-trivial-macros/)

[Explainshell - match command-line arguments to their help text](https://explainshell.com/)

[Overview of the Compiler](https://rustc-dev-guide.rust-lang.org/overview.html)

[Use borrowed types for arguments - Rust Design Patterns](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)

[A Minimal Rust Kernel | Writing an OS in Rust](https://os.phil-opp.com/minimal-rust-kernel/)

[Типы в языках программирования](http://prog.tversu.ru/library/tapl.pdf)