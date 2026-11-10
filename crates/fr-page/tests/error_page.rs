use fr_dom::query::text_content::text_content;
use fr_page::load::error_page::error_page;
use fr_page::load::escape_html::escape_html;
use url::Url;

#[test]
fn escaping_handles_markup_characters() {
    assert_eq!(
        escape_html("<a href=\"x\">&</a>"),
        "&lt;a href=&quot;x&quot;&gt;&amp;&lt;/a&gt;"
    );
}

#[test]
fn escaping_keeps_plain_text() {
    assert_eq!(escape_html("hello мир"), "hello мир");
}

#[test]
fn an_error_page_shows_the_address_and_the_message() {
    let url = Url::parse("https://example.com/missing").unwrap();

    let page = error_page(url.clone(), "connection refused");

    let text = text_content(&page.document.borrow(), page.document.borrow().root());
    assert!(text.contains("Cannot open this page"));
    assert!(text.contains("https://example.com/missing"));
    assert!(text.contains("connection refused"));
    assert_eq!(page.url, url);
}

#[test]
fn markup_in_the_message_stays_text() {
    let url = Url::parse("https://example.com/").unwrap();

    let page = error_page(url, "<script>alert(1)</script>");

    let text = text_content(&page.document.borrow(), page.document.borrow().root());
    assert!(text.contains("<script>alert(1)</script>"));
}
