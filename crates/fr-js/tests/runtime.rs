mod support;

use fr_dom::tree::document::Document;
use fr_dom::tree::shared_document::SharedDocument;
use fr_js::runtime::js_runtime::JsRuntime;
use support::Page;

#[test]
fn a_script_can_compute_and_log() {
    let mut page = Page::new("<p></p>");

    page.run("console.log(1 + 2, 'items')");

    assert_eq!(page.logs(), vec!["[log] 3 items"]);
}

#[test]
fn log_levels_are_kept() {
    let mut page = Page::new("<p></p>");

    page.run("console.warn('careful'); console.error('broken'); console.info('fyi')");

    assert_eq!(
        page.logs(),
        vec!["[warn] careful", "[error] broken", "[info] fyi"]
    );
}

#[test]
fn objects_are_logged_as_json() {
    let mut page = Page::new("<p></p>");

    page.run("console.log({ a: 1, b: [2, 3] })");

    assert_eq!(page.logs(), vec![r#"[log] {"a":1,"b":[2,3]}"#]);
}

#[test]
fn a_thrown_error_is_returned_with_its_message() {
    let mut page = Page::new("<p></p>");

    let failure = page
        .runtime
        .run_script("throw new Error('boom')")
        .unwrap_err();

    assert!(failure.message().contains("boom"));
}

#[test]
fn a_syntax_error_is_reported() {
    let mut page = Page::new("<p></p>");

    assert!(page.runtime.run_script("let = = ;").is_err());
}

#[test]
fn a_failing_script_does_not_break_the_next_one() {
    let mut page = Page::new("<p></p>");

    let _ = page.runtime.run_script("missingFunction()");
    page.run("console.log('still alive')");

    assert_eq!(page.logs(), vec!["[log] still alive"]);
}

#[test]
fn variables_live_across_scripts() {
    let mut page = Page::new("<p></p>");

    page.run("var counter = 40");
    page.run("counter += 2; console.log(counter)");

    assert_eq!(page.logs(), vec!["[log] 42"]);
}

#[test]
fn promises_run_their_callbacks() {
    let mut page = Page::new("<p></p>");

    page.run("Promise.resolve(7).then(function (value) { console.log('got', value); })");

    assert_eq!(page.logs(), vec!["[log] got 7"]);
}

#[test]
fn modern_syntax_is_available() {
    let mut page = Page::new("<p></p>");

    page.run("const square = (n) => n * n; class Box { constructor(v) { this.v = v; } } console.log(square(4), new Box(5).v, `t${1 + 1}`)");

    assert_eq!(page.logs(), vec!["[log] 16 5 t2"]);
}

#[test]
fn the_location_is_set_to_the_page_address() {
    let mut page = Page::new("<p></p>");

    page.run("console.log(location.href, window === globalThis, typeof document)");

    assert_eq!(
        page.logs(),
        vec!["[log] https://example.test/page true object"]
    );
}

#[test]
fn local_storage_keeps_values() {
    let mut page = Page::new("<p></p>");

    page.run("localStorage.setItem('k', 'v'); console.log(localStorage.getItem('k'), localStorage.getItem('x'), localStorage.length)");

    assert_eq!(page.logs(), vec!["[log] v null 1"]);
}

#[test]
fn a_runtime_without_nodes_still_starts() {
    let document = SharedDocument::new(Document::new());

    let runtime = JsRuntime::new(document, "about:blank");

    assert!(runtime.is_ok());
}
