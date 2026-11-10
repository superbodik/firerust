mod support;

use support::Page;

#[test]
fn an_onclick_attribute_runs_on_click() {
    let mut page =
        Page::new("<button id=b onclick=\"console.log('attr', event.type)\">go</button>");

    let button = page.id("b");
    page.runtime.dispatch_event(button, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] attr click"]);
}

#[test]
fn this_is_the_element_inside_an_attribute_handler() {
    let mut page = Page::new("<button id=b onclick=\"console.log(this.id)\">go</button>");

    let button = page.id("b");
    page.runtime.dispatch_event(button, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] b"]);
}

#[test]
fn returning_false_prevents_the_default_action() {
    let mut page = Page::new("<a id=a href='/x' onclick=\"return false\">x</a>");

    let anchor = page.id("a");
    let allowed = page.runtime.dispatch_event(anchor, "click", None).unwrap();

    assert!(!allowed);
}

#[test]
fn an_attribute_handler_bubbles_like_a_listener() {
    let mut page = Page::new(
        "<div id=d onclick=\"console.log('outer')\"><button id=b onclick=\"console.log('inner')\">go</button></div>",
    );

    let button = page.id("b");
    page.runtime.dispatch_event(button, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] inner", "[log] outer"]);
}

#[test]
fn a_broken_attribute_handler_is_logged() {
    let mut page = Page::new("<button id=b onclick=\"let = ;\">go</button>");

    let button = page.id("b");
    page.runtime.dispatch_event(button, "click", None).unwrap();

    assert!(page.logs()[0].starts_with("[error] Uncaught"));
}

#[test]
fn changing_the_attribute_changes_the_handler() {
    let mut page = Page::new("<button id=b onclick=\"console.log('old')\">go</button>");
    let button = page.id("b");
    page.runtime.dispatch_event(button, "click", None).unwrap();

    page.run("document.getElementById('b').setAttribute('onclick', \"console.log('new')\")");
    page.runtime.dispatch_event(button, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] old", "[log] new"]);
}
