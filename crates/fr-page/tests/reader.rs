mod support;

use fr_html::parse::parse_html::parse_html;
use fr_page::reader::build_reader_page::build_reader_page;
use fr_page::reader::extract_reader_content::extract_reader_content;
use fr_page::reader::is_safe_reference::is_safe_reference;
use fr_theme::kind::theme_kind::ThemeKind;
use support::{layout_of, texts};
use url::Url;

fn sentence(topic: &str) -> String {
    format!("{topic} ").repeat(120)
}

fn extract(html: &str) -> Option<fr_page::reader::reader_content::ReaderContent> {
    extract_reader_content(&parse_html(html))
}

#[test]
fn the_article_is_kept_and_the_page_furniture_is_dropped() {
    let html = format!(
        "<html><head><title>My article</title></head><body>\
         <nav><a href='/'>Home</a> <a href='/about'>About</a></nav>\
         <article><h1>Heading</h1><p>{}</p><p>{}</p></article>\
         <footer>Copyright footer text</footer></body></html>",
        sentence("alpha"),
        sentence("beta")
    );

    let content = extract(&html).expect("content");

    assert_eq!(content.title, "My article");
    assert!(content.html.contains("alpha alpha"));
    assert!(content.html.contains("beta beta"));
    assert!(content.html.contains("<h1>Heading</h1>"));
    assert!(!content.html.contains("Home"));
    assert!(!content.html.contains("Copyright"));
}

#[test]
fn scripts_styles_and_forms_never_reach_the_reader() {
    let html = format!(
        "<body><article><p>{}</p><script>var secret = 1;</script>\
         <style>.x {{ color: red }}</style><form><input name=q><button>Go</button></form>\
         <iframe src='https://ads.example'></iframe></article></body>",
        sentence("gamma")
    );

    let content = extract(&html).expect("content");

    assert!(!content.html.contains("secret"));
    assert!(!content.html.contains("color: red"));
    assert!(!content.html.contains("<input"));
    assert!(!content.html.contains("<iframe"));
    assert!(!content.html.contains("<button"));
}

#[test]
fn dangerous_links_and_images_lose_their_address() {
    let html = format!(
        "<article><p>{}</p>\
         <p><a href='javascript:evil()'>bad link</a> <a href='/ok'>good link</a> \
         <a href='https://example.com/x'>web link</a> <a href='mailto:a@b.c'>mail</a></p>\
         <img src='data:image/png;base64,AAAA'><img src='/pic.png' alt='A picture'></article>",
        sentence("delta")
    );

    let content = extract(&html).expect("content");

    assert!(!content.html.contains("javascript"));
    assert!(content.html.contains("bad link"));
    assert!(content.html.contains("<a href=\"/ok\">good link</a>"));
    assert!(content.html.contains("href=\"https://example.com/x\""));
    assert!(content.html.contains("href=\"mailto:a@b.c\""));
    assert!(!content.html.contains("data:image"));
    assert!(
        content
            .html
            .contains("<img src=\"/pic.png\" alt=\"A picture\">")
    );
}

#[test]
fn references_are_checked_by_scheme() {
    assert!(is_safe_reference("/path/page.html"));
    assert!(is_safe_reference("page.html?x=a:b"));
    assert!(is_safe_reference("#section"));
    assert!(is_safe_reference("HTTPS://EXAMPLE.COM"));
    assert!(!is_safe_reference("javascript:alert(1)"));
    assert!(!is_safe_reference("  JaVaScRiPt:alert(1)"));
    assert!(!is_safe_reference("data:text/html,<b>x</b>"));
    assert!(!is_safe_reference("file:///C:/secret"));
    assert!(!is_safe_reference(""));
}

#[test]
fn without_semantic_tags_the_text_heavy_block_wins() {
    let html = format!(
        "<body><div class='sidebar'><ul>{}</ul></div>\
         <div class='post'><p>{}</p><p>{}</p><p>{}</p></div></body>",
        "<li><a href='/x'>Short link</a></li>".repeat(12),
        sentence("epsilon"),
        sentence("zeta"),
        sentence("eta")
    );

    let content = extract(&html).expect("content");

    assert!(content.html.contains("epsilon epsilon"));
    assert!(content.html.contains("eta eta"));
    assert!(!content.html.contains("Short link"));
}

#[test]
fn a_list_made_of_links_inside_the_article_is_dropped() {
    let html = format!(
        "<article><p>{}</p><ul><li><a href='/a'>One link</a></li>\
         <li><a href='/b'>Two link</a></li><li><a href='/c'>Three link</a></li></ul>\
         <ul><li>A plain list item with real words in it</li><li>Another plain item</li></ul></article>",
        sentence("theta")
    );

    let content = extract(&html).expect("content");

    assert!(!content.html.contains("One link"));
    assert!(content.html.contains("A plain list item"));
}

#[test]
fn noisy_classes_and_hidden_elements_are_dropped() {
    let html = format!(
        "<article><p>{}</p><div class='share-buttons'>Share this now</div>\
         <div id='comments'>Nice post!</div><div hidden>Hidden text</div>\
         <div style='display: none'>Also hidden</div><sup class='reference'>[1]</sup></article>",
        sentence("iota")
    );

    let content = extract(&html).expect("content");

    assert!(!content.html.contains("Share this"));
    assert!(!content.html.contains("Nice post"));
    assert!(!content.html.contains("Hidden text"));
    assert!(!content.html.contains("Also hidden"));
    assert!(!content.html.contains("[1]"));
}

#[test]
fn a_short_page_has_no_reader_view() {
    assert!(extract("<body><p>Just a few words.</p></body>").is_none());
    assert!(extract("<body><nav><a href='/'>x</a></nav></body>").is_none());
}

#[test]
fn only_the_first_twenty_images_are_kept() {
    let pictures = "<img src='/p.png'>".repeat(30);
    let html = format!("<article><p>{}</p>{pictures}</article>", sentence("kappa"));

    let content = extract(&html).expect("content");

    assert_eq!(content.html.matches("<img").count(), 20);
}

#[test]
fn the_title_falls_back_to_the_first_heading() {
    let html = format!(
        "<html><body><article><h1>The  big\n heading</h1><p>{}</p></article></body></html>",
        sentence("lambda")
    );

    let content = extract(&html).expect("content");

    assert_eq!(content.title, "The big heading");
}

#[test]
fn very_deep_nesting_does_not_overflow_the_stack() {
    let depth = 2_000;
    let html = format!(
        "<article>{}<p>{}</p>{}</article>",
        "<div>".repeat(depth),
        sentence("mu"),
        "</div>".repeat(depth)
    );

    let content = extract(&html).expect("content");

    assert!(content.html.contains("mu mu"));
}

#[test]
fn the_reader_page_is_a_real_page_with_the_article_text() {
    let html = format!(
        "<html><head><title>Story</title></head><body><nav>Menu</nav>\
         <article><p>{}</p></article></body></html>",
        sentence("nu")
    );
    let document = parse_html(&html);
    let url = Url::parse("https://news.example/story").unwrap();

    let page = build_reader_page(&document, &url, ThemeKind::Light).expect("page");

    assert_eq!(page.title.as_deref(), Some("Story"));
    assert_eq!(page.url, url);
    let shown = texts(&layout_of(&page, 800.0)).join(" ");
    assert!(shown.contains("nu nu"));
    assert!(shown.contains("news.example"));
    assert!(!shown.contains("Menu"));
}

#[test]
fn the_reader_page_follows_the_theme() {
    let html = format!("<article><p>{}</p></article>", sentence("xi"));
    let document = parse_html(&html);
    let url = Url::parse("https://news.example/").unwrap();

    let light = build_reader_page(&document, &url, ThemeKind::Light).expect("light");
    let dark = build_reader_page(&document, &url, ThemeKind::Dark).expect("dark");

    assert_ne!(
        layout_of(&light, 800.0).background,
        layout_of(&dark, 800.0).background
    );
}

#[test]
fn infoboxes_are_left_out_of_the_reader() {
    let html = format!(
        "<article><table class='infobox'><tr><td>Born in a small town</td></tr></table>\
         <p>{}</p></article>",
        sentence("omicron")
    );

    let content = extract(&html).expect("content");

    assert!(!content.html.contains("small town"));
    assert!(content.html.contains("omicron omicron"));
}

#[test]
fn the_page_heading_is_not_repeated_when_the_article_has_its_own() {
    let html = format!(
        "<html><head><title>Site - Story</title></head><body><article><h1>Story</h1><p>{}</p></article></body></html>",
        sentence("pi")
    );
    let document = parse_html(&html);
    let url = Url::parse("https://news.example/story").unwrap();

    let page = build_reader_page(&document, &url, ThemeKind::Light).expect("page");

    let shown = texts(&layout_of(&page, 800.0));
    assert_eq!(
        shown.iter().filter(|line| line.contains("Story")).count(),
        1
    );
}

#[test]
fn a_heading_is_added_when_the_article_has_none() {
    let html = format!(
        "<html><head><title>Plain title</title></head><body><article><p>{}</p></article></body></html>",
        sentence("rho")
    );
    let document = parse_html(&html);
    let url = Url::parse("https://news.example/plain").unwrap();

    let page = build_reader_page(&document, &url, ThemeKind::Light).expect("page");

    let shown = texts(&layout_of(&page, 800.0)).join(" ");
    assert!(shown.contains("Plain title"));
}
