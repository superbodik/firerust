<div align="center">

# firerust

**A lightweight web browser engine written from scratch in Rust.**

Aiming for a smaller memory footprint than Firefox on simple and medium-sized sites.

[English](README.md) · [Русский](README.ru.md)

![Rust](https://img.shields.io/badge/Rust-1.96-b7410e?logo=rust&logoColor=white)
![Edition](https://img.shields.io/badge/edition-2024-informational)
![Platform](https://img.shields.io/badge/platform-Windows%2011-0078d4)
![Status](https://img.shields.io/badge/status-early%20development-orange)

</div>

---

## About

firerust is not a fork of Firefox or Chromium. It is an own engine: HTML parsing, CSS cascade, layout, painting and networking are separate small crates, and only low-level building blocks (windowing, text shaping, image decoding, flexbox math) come from existing libraries.

The goal is simple: open simple and medium-complexity sites while using noticeably less RAM than Firefox does on the same pages.

> The project is at an early stage. It is a page viewer, not yet a full browser. Memory comparisons with Firefox have not been measured yet.

## What works today

- Opens a local file or an `http` / `https` address
- Loads and applies CSS from `<style>`, `<link>` and `style=""` with a real cascade: origin, specificity, order, `!important`, inheritance
- Block layout with margin, padding, border, margin collapsing, `width`, `min-width`, `max-width`, `box-sizing`
- Inline text with line wrapping, mixed colours, weights, sizes and italics
- Images: PNG, JPEG, GIF, WebP
- Lists with markers
- Flexbox (via [taffy](https://github.com/DioxusLabs/taffy))
- Tables with content-based column widths and `colspan`
- Re-layout when the window is resized
- Navigation: address bar, back / forward / reload buttons, clickable links, history that remembers the scroll position
- Scrolling with the wheel, arrow keys, Page Up / Page Down, Space, Home and End, with a scrollbar thumb
- A readable error page when a page cannot be opened
- Tabs: new tab, close, switch; background tabs beyond the two most recently used drop their page from memory and are fetched again when selected
- A shared byte-limited image cache and a memory panel (`Ctrl+M`)
- Safety: a web page cannot read local files or link to them, a secure page cannot load insecure images, styles or scripts, and a `Content-Security-Policy` meta tag limits where scripts, styles and images come from
- Networking: page character sets (windows-1251, KOI8-R and the rest of the web set), gzip and brotli, redirects, cookies, POST requests
- Forms: text fields, passwords, checkboxes, radio buttons, selects, text areas and buttons you can click and type into, Tab between them, GET and POST submission, `input`, `change` and `submit` events for scripts
- JavaScript on [Boa](https://github.com/boa-dev/boa): inline and external scripts, `DOMContentLoaded` and `load`, `document` and element APIs (queries, creating and moving nodes, `textContent`, `innerHTML`, attributes, `classList`, `style`), events with capture and bubbling phases, `onclick` attributes and properties, `setTimeout`, `setInterval`, `requestAnimationFrame`, `console`, `localStorage`
- Deeply nested documents are safe: layout runs on a large stack and the tree is freed iteratively

Pages that load and lay out: Hacker News, example.com, CERN, NPR, Lobsters.

Keys: `Ctrl+L` or `F6` focus the address bar, `Enter` opens, `Esc` cancels, `Alt+Left` and `Alt+Right` go back and forward, `F5` or `Ctrl+R` reloads, `Ctrl+T` opens a tab, `Ctrl+W` closes it, `Ctrl+Tab` and `Ctrl+Shift+Tab` switch tabs, `Ctrl+M` toggles the memory panel. Shortcuts use physical keys, so they work on any keyboard layout.

## Quick start

Requirements: Rust 1.96 (MSVC toolchain), Windows 11.

```
git clone https://github.com/superbodik/firerust.git
cd firerust
cargo run -p fr-shell -- tests/pages/blog.html
```

A small linked site for trying navigation:

```
cargo run -p fr-shell -- tests/pages/site/index.html
```

JavaScript demos, a counter with a timer and a drop-down menu:

```
cargo run -p fr-shell -- tests/pages/js_counter.html
cargo run -p fr-shell -- tests/pages/js_menu.html
```

Forms and a log-in flow against the bundled test server (in a second terminal run `cargo run -p fr-test-server`, then open `http://127.0.0.1:8080/login`, user `ada`, password `lovelace`):

```
cargo run -p fr-test-server -- 127.0.0.1:8080
cargo run -p fr-shell -- tests/pages/forms.html
```

An address works too, and several addresses open as tabs:

```
cargo run -p fr-shell -- https://example.com
cargo run -p fr-shell -- tests/pages/plain.html tests/pages/blog.html
```

Bundled test pages live in [tests/pages](tests/pages): `plain`, `nested`, `entities`, `cyrillic`, `blog`, `images`, `columns`, `tables` and the linked `site/`.

## Memory

The point of the project. First measurements on Windows 11 with a release build, against Firefox started with a fresh temporary profile (private working set, all processes of the browser summed):

| Case | firerust | Firefox |
|---|---|---|
| One simple page | 5.6 to 6.7 MB, 1 process | 310 to 350 MB, 11 processes |
| One page with scripts (a counter, a menu) | 7.5 MB, 1 process | 325 to 330 MB, 11 processes |
| 8 tabs, scripts running (local pages, example.com, Hacker News, lobste.rs, rust-lang.org) | 16.8 MB, 1 process | 420 to 590 MB, 15 processes |

This is not a fair fight yet: firerust runs only a small part of the web platform (no `fetch`, no layout APIs for scripts), only part of HTML and CSS, no web fonts and no media, so these numbers are a lower bound and will grow as features arrive. The script engine itself costs about 2 MB per page that uses it. The benchmark script is [tools/mem-bench/mem-bench.ps1](tools/mem-bench/mem-bench.ps1).

## Architecture

Every crate has one responsibility. Every file holds one type or one function.

```
fr-shell ─ binary: entry point and scenes
 ├─ fr-window   window (winit), input events, frame output (softbuffer)
 ├─ fr-ui       scroll state, address field, toolbar geometry (no window needed)
 ├─ fr-security page safety rules: mixed content, local files, content security policy
 ├─ fr-js       JavaScript runtime on Boa: DOM, events, timers
 ├─ fr-form     form fields, GET and POST requests, focus order
 ├─ fr-tab      tabs, unloading of background tabs, memory estimates
 ├─ fr-history  back / forward stack with scroll positions
 ├─ fr-cache    byte-limited LRU cache
 ├─ fr-mem      memory of the running process
 ├─ fr-page     address → load → parse → styles → images
 │   ├─ fr-net     address resolution, file / http / https loading
 │   ├─ fr-html    HTML → DOM (html5ever)
 │   ├─ fr-css     parsing, selectors, cascade, computed styles
 │   └─ fr-image   PNG / JPEG / GIF / WebP decoding
 ├─ fr-layout   box tree: blocks, inline text, images, flex, tables, hit testing
 ├─ fr-paint    display list and painting
 ├─ fr-raster   canvas (tiny-skia) and pixel export
 ├─ fr-text     fonts and text shaping (cosmic-text)
 ├─ fr-dom      DOM tree in an arena, iterative traversal
 └─ fr-geom     Point, Size, Rect, Rgba
```

Design choices that serve the memory goal:

- The DOM lives in a single arena: nodes in one `Vec`, links through 4-byte `NodeId`
- Traversal and teardown are iterative, with no recursion on document depth
- Computed styles and layout boxes are flat, compact data

## Development

```
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

About 640 tests pass. One network test is run manually:

```
cargo test -p fr-page --test real_sites -- --ignored
```

Code rules: no comments in code, small files grouped in folders, no `unwrap` or `expect` in library code, no `unsafe` without a separate decision.

## Roadmap

| Stage | Topic | Status |
|---|---|---|
| 0 | Preparation | done |
| 1 | Window and text | done |
| 2 | Loading and DOM | done |
| 3 | CSS and cascade | done |
| 4 | Layout | done |
| 5 | Navigation and UI: address bar, back / forward, link clicks, scrolling | done |
| 6 | Tabs and memory | done |
| 7 | JavaScript | done |
| 8 | Network and forms | done |
| 9 | Security and polish | next |

## Known limitations

- Content security policy is read only from the meta tag, not from response headers; no nonces or hashes
- Form controls take a line of their own, a select changes on click without a drop-down, no file upload, no `reset` button, no form validation; cookies live only in memory; going back after a POST asks the address again with GET
- Scripts and the final page build still run on the window thread, so a heavy page can stall the window while it is built; several start addresses after the first load one after another
- No page zoom yet; links to `#fragment` do not scroll to the anchor
- Back and forward reload the page, there is no page cache yet
- The last tab cannot be closed
- Inline images become their own blocks; margins, padding and borders on inline elements are ignored; no underline
- No `float` or `position`
- Tables have no `valign` or `border-spacing`
- UTF-8 pages only
- JavaScript: no `fetch` or `XMLHttpRequest`, no `document.write`, scripts see no element sizes, only clicks and timers reach pages (no hover or keyboard events yet), the whole document is restyled after every change

## Built with

[winit](https://github.com/rust-windowing/winit) · [softbuffer](https://github.com/rust-windowing/softbuffer) · [tiny-skia](https://github.com/linebender/tiny-skia) · [cosmic-text](https://github.com/pop-os/cosmic-text) · [html5ever](https://github.com/servo/html5ever) · [cssparser](https://github.com/servo/rust-cssparser) · [taffy](https://github.com/DioxusLabs/taffy) · [image](https://github.com/image-rs/image) · [ureq](https://github.com/algesten/ureq) · [url](https://github.com/servo/rust-url)
