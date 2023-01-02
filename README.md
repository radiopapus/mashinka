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
[Дока по Rust](https://doc.rust-lang.org/book/)

[Типы в языках программирования](http://prog.tversu.ru/library/tapl.pdf)

https://cheats.rs/

https://paste.rs/web
