use std::path::PathBuf;

use fr_dom::node::node_id::NodeId;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::edge::Edge;
use fr_page::load::build_page::build_page;
use fr_page::load::load_page::load_page;
use fr_page::load::page::Page;
use fr_page::script::collect_scripts::collect_scripts;
use fr_page::script::script_source::ScriptSource;
use url::Url;

fn address() -> Url {
    Url::parse("https://example.test/dir/page.html").unwrap()
}

fn page_of(html: &str) -> Page {
    build_page(html, address())
}

fn text_of(page: &Page, wanted: &str) -> String {
    let document = page.document.borrow();
    let id = find(&document, wanted);
    text_content(&document, id)
}

fn find(document: &fr_dom::tree::document::Document, wanted: &str) -> NodeId {
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

#[test]
fn a_page_without_scripts_has_no_runtime() {
    let page = page_of("<p>plain</p>");

    assert!(page.runtime.is_none());
}

#[test]
fn an_inline_script_changes_the_document_before_styling() {
    let page =
        page_of("<p id=t>old</p><script>document.getElementById('t').textContent = 'new'</script>");

    assert_eq!(text_of(&page, "t"), "new");
    assert!(page.runtime.is_some());
}

#[test]
fn scripts_run_in_document_order_and_share_globals() {
    let page = page_of(
        "<p id=t></p><script>var n = 1;</script><script>n += 1; document.getElementById('t').textContent = 'n=' + n;</script>",
    );

    assert_eq!(text_of(&page, "t"), "n=2");
}

#[test]
fn a_failing_script_does_not_stop_the_next_one() {
    let page = page_of(
        "<p id=t>x</p><script>missing()</script><script>document.getElementById('t').textContent = 'ok'</script>",
    );

    assert_eq!(text_of(&page, "t"), "ok");
}

#[test]
fn dom_content_loaded_handlers_run_after_all_scripts() {
    let page = page_of(
        "<p id=t></p><script>document.addEventListener('DOMContentLoaded', function () { document.getElementById('t').textContent = 'ready ' + document.readyState; });</script>",
    );

    assert_eq!(text_of(&page, "t"), "ready interactive");
}

#[test]
fn window_load_handlers_run() {
    let page = page_of(
        "<p id=t></p><script>window.onload = function () { document.getElementById('t').textContent = 'loaded'; };</script>",
    );

    assert_eq!(text_of(&page, "t"), "loaded");
}

#[test]
fn scripts_of_other_types_are_not_run() {
    let page = page_of(
        "<p id=t>x</p><script type='text/template'>document.getElementById('t').textContent = 'ran'</script><script type='module'>document.getElementById('t').textContent = 'module'</script>",
    );

    assert_eq!(text_of(&page, "t"), "x");
}

#[test]
fn an_inline_event_attribute_starts_the_runtime() {
    let page = page_of("<button id=b onclick=\"this.textContent = 'x'\">go</button>");

    assert!(page.runtime.is_some());
}

#[test]
fn an_external_script_is_loaded_before_the_next_inline_one() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/pages/js");

    let page = load_page("external.html", &directory).expect("page");

    assert_eq!(text_of(&page, "from-file"), "loaded from a file");
    assert_eq!(text_of(&page, "inline"), "inline ran after the file");
}

#[test]
fn a_missing_external_script_is_skipped() {
    let page = page_of(
        "<p id=t>x</p><script src='file:///no/such/file.js'></script><script>document.getElementById('t').textContent = 'after'</script>",
    );

    assert_eq!(text_of(&page, "t"), "after");
}

#[test]
fn scripts_are_collected_in_document_order() {
    let page = page_of(
        "<script>one()</script><script src='lib/a.js'></script><script>two()</script><script></script>",
    );

    let scripts = collect_scripts(&page.document.borrow(), &address());

    assert_eq!(
        scripts,
        vec![
            ScriptSource::Inline("one()".to_owned()),
            ScriptSource::External(Url::parse("https://example.test/dir/lib/a.js").unwrap()),
            ScriptSource::Inline("two()".to_owned()),
        ]
    );
}

#[test]
fn the_script_sees_the_page_address() {
    let page = page_of(
        "<p id=t></p><script>document.getElementById('t').textContent = location.href</script>",
    );

    assert_eq!(text_of(&page, "t"), "https://example.test/dir/page.html");
}

#[test]
fn elements_added_by_a_script_are_styled() {
    let page = page_of(
        "<style>.big { font-size: 40px }</style><script>var d = document.createElement('div'); d.className = 'big'; d.id = 'made'; d.textContent = 'hi'; document.body.appendChild(d);</script>",
    );

    let document = page.document.borrow();
    let id = find(&document, "made");
    let style = page.styles.get(id).expect("style for the new element");
    assert_eq!(style.font_size, 40.0);
}
