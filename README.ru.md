<div align="center">

# firerust

**Лёгкий браузерный движок, написанный на Rust с нуля.**

Цель: расходовать меньше ОЗУ, чем Firefox, на простых и средних сайтах.

[English](README.md) · [Русский](README.ru.md)

![Rust](https://img.shields.io/badge/Rust-1.96-b7410e?logo=rust&logoColor=white)
![Edition](https://img.shields.io/badge/edition-2024-informational)
![Platform](https://img.shields.io/badge/platform-Windows%2011-0078d4)
![Status](https://img.shields.io/badge/status-ранняя%20разработка-orange)

</div>

---

## О проекте

firerust — не форк Firefox или Chromium, а собственный движок: разбор HTML, каскад CSS, вёрстка, отрисовка и сеть — отдельные мелкие крейты. Из готовых библиотек взяты только низкоуровневые части: окно, шейпинг текста, декодирование картинок, математика flexbox.

Цель простая: открывать простые и средние по сложности сайты и тратить заметно меньше ОЗУ, чем Firefox на тех же страницах.

> Проект на ранней стадии. Пока это просмотрщик страниц, а не полноценный браузер. Сравнение по памяти с Firefox ещё не измерялось.

## Что работает сейчас

- Открывает локальный файл или адрес `http` / `https`
- Подтягивает и применяет CSS из `<style>`, `<link>` и `style=""` с настоящим каскадом: источник, специфичность, порядок, `!important`, наследование
- Блочная вёрстка: margin, padding, border, схлопывание отступов, `width`, `min-width`, `max-width`, `box-sizing`
- Строчный текст с переносом строк, разными цветами, весами, размерами и курсивом
- Картинки: PNG, JPEG, GIF, WebP
- Списки с маркерами
- Flexbox (через [taffy](https://github.com/DioxusLabs/taffy))
- Таблицы: ширины столбцов по содержимому, `colspan`
- Пересчёт вёрстки при изменении размера окна
- Глубокая вложенность безопасна: вёрстка идёт на большом стеке, дерево освобождается итеративно

Страницы, которые загружаются и раскладываются: Hacker News, example.com, CERN, NPR, Lobsters.

## Быстрый старт

Нужно: Rust 1.96 (тулчейн MSVC), Windows 11.

```
git clone https://github.com/superbodik/firerust.git
cd firerust
cargo run -p fr-shell -- tests/pages/blog.html
```

Адрес тоже подойдёт:

```
cargo run -p fr-shell -- https://example.com
```

Тестовые страницы лежат в [tests/pages](tests/pages): `plain`, `nested`, `entities`, `cyrillic`, `blog`, `images`, `columns`, `tables`.

## Архитектура

У каждого крейта одна ответственность. В каждом файле один тип или одна функция.

```
fr-shell ─ бинарник: точка входа и сцены
 ├─ fr-window   окно (winit) и вывод кадра (softbuffer)
 ├─ fr-page     адрес → загрузка → разбор → стили → картинки
 │   ├─ fr-net     разбор адреса, загрузка file / http / https
 │   ├─ fr-html    HTML → DOM (html5ever)
 │   ├─ fr-css     разбор, селекторы, каскад, вычисленные стили
 │   └─ fr-image   декодирование PNG / JPEG / GIF / WebP
 ├─ fr-layout   дерево боксов: блоки, текст, картинки, flex, таблицы, поиск под курсором
 ├─ fr-paint    список команд отрисовки и рисование
 ├─ fr-raster   холст (tiny-skia) и экспорт пикселей
 ├─ fr-text     шрифты и шейпинг текста (cosmic-text)
 ├─ fr-dom      DOM-дерево на арене, обход без рекурсии
 └─ fr-geom     Point, Size, Rect, Rgba
```

Решения, которые работают на цель по памяти:

- DOM лежит в одной арене: узлы в одном `Vec`, связи через 4-байтовые `NodeId`
- Обход и освобождение итеративные, без рекурсии по глубине документа
- Вычисленные стили и боксы вёрстки — плоские компактные данные

## Разработка

```
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Проходит около 255 тестов. Один сетевой тест запускается вручную:

```
cargo test -p fr-page --test real_sites -- --ignored
```

Правила кода: без комментариев, мелкие файлы по папкам, без `unwrap` и `expect` в библиотечном коде, без `unsafe` без отдельного решения.

## Дорожная карта

| Этап | Тема | Статус |
|---|---|---|
| 0 | Подготовка | готово |
| 1 | Окно и текст | готово |
| 2 | Загрузка и DOM | готово |
| 3 | CSS и каскад | готово |
| 4 | Вёрстка | готово |
| 5 | Навигация и UI: адресная строка, назад / вперёд, клики по ссылкам, прокрутка, зум | следующий |
| 6 | Вкладки и память | запланирован |
| 7 | JavaScript | запланирован |
| 8 | Сеть и формы | запланирован |
| 9 | Безопасность и полировка | запланирован |

## Известные ограничения

- Пока нет прокрутки, кликов по ссылкам и адресной строки (этап 5)
- Строчная картинка становится отдельным блоком; отступы, рамки и padding у строчных элементов игнорируются; нет подчёркивания
- Нет `float` и `position`
- В таблицах нет `valign` и `border-spacing`
- Только страницы в UTF-8
- Нет JavaScript

## Собрано на

[winit](https://github.com/rust-windowing/winit) · [softbuffer](https://github.com/rust-windowing/softbuffer) · [tiny-skia](https://github.com/linebender/tiny-skia) · [cosmic-text](https://github.com/pop-os/cosmic-text) · [html5ever](https://github.com/servo/html5ever) · [cssparser](https://github.com/servo/rust-cssparser) · [taffy](https://github.com/DioxusLabs/taffy) · [image](https://github.com/image-rs/image) · [ureq](https://github.com/algesten/ureq) · [url](https://github.com/servo/rust-url)
