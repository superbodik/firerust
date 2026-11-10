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
- Deeply nested documents are safe: layout runs on a large stack and the tree is freed iteratively

Pages that load and lay out: Hacker News, example.com, CERN, NPR, Lobsters.

Keys: `Ctrl+L` or `F6` focus the address bar, `Enter` opens, `Esc` cancels, `Alt+Left` and `Alt+Right` go back and forward, `F5` or `Ctrl+R` reloads.

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

An address works too:

```
cargo run -p fr-shell -- https://example.com
```

Bundled test pages live in [tests/pages](tests/pages): `plain`, `nested`, `entities`, `cyrillic`, `blog`, `images`, `columns`, `tables` and the linked `site/`.

## Architecture

Every crate has one responsibility. Every file holds one type or one function.

```
fr-shell ─ binary: entry point and scenes
 ├─ fr-window   window (winit), input events, frame output (softbuffer)
 ├─ fr-ui       scroll state, address field, toolbar geometry (no window needed)
 ├─ fr-history  back / forward stack with scroll positions
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

About 330 tests pass. One network test is run manually:

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
| 6 | Tabs and memory | next |
| 7 | JavaScript | planned |
| 8 | Network and forms | planned |
| 9 | Security and polish | planned |

## Known limitations

- Loading blocks the window while a page is fetched
- No page zoom yet; links to `#fragment` do not scroll to the anchor
- Back and forward reload the page, there is no page cache yet
- Inline images become their own blocks; margins, padding and borders on inline elements are ignored; no underline
- No `float` or `position`
- Tables have no `valign` or `border-spacing`
- UTF-8 pages only
- No JavaScript

## Built with

[winit](https://github.com/rust-windowing/winit) · [softbuffer](https://github.com/rust-windowing/softbuffer) · [tiny-skia](https://github.com/linebender/tiny-skia) · [cosmic-text](https://github.com/pop-os/cosmic-text) · [html5ever](https://github.com/servo/html5ever) · [cssparser](https://github.com/servo/rust-cssparser) · [taffy](https://github.com/DioxusLabs/taffy) · [image](https://github.com/image-rs/image) · [ureq](https://github.com/algesten/ureq) · [url](https://github.com/servo/rust-url)
