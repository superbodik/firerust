use fr_dom::node::node_id::NodeId;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_page::link::resolve_link::resolve_link;
use fr_page::load::build_page::build_page;
use fr_page::load::page::Page;
use url::Url;

fn page_of(html: &str, address: &str) -> Page {
    build_page(html, Url::parse(address).unwrap())
}

fn find(document: &Document, wanted: &str) -> NodeId {
    document
        .walk(document.root())
        .find_map(|edge| match edge {
            Edge::Enter(id) => {
                let element = document.node(id)?.kind().as_element()?;
                (element.attribute("id") == Some(wanted)).then_some(id)
            }
            Edge::Leave(_) => None,
        })
        .expect("element with the id")
}

fn text_of(page: &Page, wanted: &str) -> String {
    let document = page.document.borrow();
    text_content(&document, find(&document, wanted))
}

const SCRIPT: &str =
    "<p id=out>before</p><script>document.getElementById('out').textContent = 'after';</script>";

#[test]
fn inline_scripts_run_without_a_policy() {
    let page = page_of(SCRIPT, "https://example.test/");

    assert_eq!(text_of(&page, "out"), "after");
}

#[test]
fn a_policy_without_unsafe_inline_stops_inline_scripts() {
    let html = format!(
        "<meta http-equiv=\"Content-Security-Policy\" content=\"script-src 'self'\">{SCRIPT}"
    );
    let page = page_of(&html, "https://example.test/");

    assert_eq!(text_of(&page, "out"), "before");
}

#[test]
fn a_policy_with_unsafe_inline_lets_inline_scripts_run() {
    let html = format!(
        "<meta http-equiv=\"Content-Security-Policy\" content=\"script-src 'unsafe-inline'\">{SCRIPT}"
    );
    let page = page_of(&html, "https://example.test/");

    assert_eq!(text_of(&page, "out"), "after");
}

#[test]
fn a_policy_without_unsafe_inline_stops_attribute_handlers() {
    let html = "<meta http-equiv=\"Content-Security-Policy\" content=\"script-src 'self'\">\
                <p id=out>before</p>\
                <button id=b onclick=\"document.getElementById('out').textContent='clicked'\">b</button>\
                <script src=\"missing.js\"></script>";
    let mut page = page_of(html, "https://example.test/");
    let button = {
        let document = page.document.borrow();
        find(&document, "b")
    };
    if let Some(runtime) = page.runtime.as_mut() {
        runtime.dispatch_event(button, "click", None).unwrap();
    }

    assert_eq!(text_of(&page, "out"), "before");
}

#[test]
fn attribute_handlers_work_without_a_policy() {
    let html = "<p id=out>before</p>\
                <button id=b onclick=\"document.getElementById('out').textContent='clicked'\">b</button>";
    let mut page = page_of(html, "https://example.test/");
    let button = {
        let document = page.document.borrow();
        find(&document, "b")
    };
    if let Some(runtime) = page.runtime.as_mut() {
        runtime.dispatch_event(button, "click", None).unwrap();
    }

    assert_eq!(text_of(&page, "out"), "clicked");
}

#[test]
fn a_web_page_cannot_link_to_a_local_file() {
    let base = Url::parse("https://example.test/").unwrap();

    assert!(resolve_link(&base, "file:///C:/secret.html").is_none());
    assert!(resolve_link(&base, "other.html").is_some());
}

#[test]
fn a_local_page_can_link_to_a_local_file() {
    let base = Url::parse("file:///C:/site/index.html").unwrap();

    assert!(resolve_link(&base, "about.html").is_some());
}
