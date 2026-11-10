use fr_dom::node::node_id::NodeId;
use fr_dom::tree::edge::Edge;
use fr_page::load::build_page::build_page;
use fr_tab::model::tab_content::TabContent;
use url::Url;

fn address() -> Url {
    Url::parse("https://example.test/").unwrap()
}

fn content_of(html: &str) -> TabContent {
    TabContent::from_page(build_page(html, address()))
}

fn find(content: &TabContent, wanted: &str) -> NodeId {
    let document = content.document.borrow();
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
fn fresh_content_is_not_stale() {
    let content = content_of("<p>hi</p>");

    assert!(!content.is_stale());
}

#[test]
fn a_script_change_makes_the_content_stale() {
    let mut content = content_of("<button id=b onclick=\"this.textContent = 'x'\">go</button>");
    let button = find(&content, "b");

    content.dispatch_event(button, "click", None);

    assert!(content.is_stale());
}

#[test]
fn refreshing_restyles_and_drops_the_layout() {
    let mut content = content_of(
        "<style>.on { color: red }</style><button id=b onclick=\"this.className = 'on'\">go</button>",
    );
    let button = find(&content, "b");
    content.dispatch_event(button, "click", None);

    let refreshed = content.refresh(&address());

    assert!(refreshed);
    assert!(!content.is_stale());
    assert!(content.layout.is_none());
    let style = content.styles.get(button).expect("style");
    assert_eq!(style.color.r, 255);
}

#[test]
fn refreshing_fresh_content_does_nothing() {
    let mut content = content_of("<p>hi</p>");

    assert!(!content.refresh(&address()));
}

#[test]
fn events_on_a_page_without_scripts_are_allowed() {
    let mut content = content_of("<a id=a href='/x'>x</a>");
    let anchor = find(&content, "a");

    assert!(content.dispatch_event(anchor, "click", None));
}

#[test]
fn a_prevented_click_is_reported() {
    let mut content = content_of("<a id=a href='/x' onclick=\"return false\">x</a>");
    let anchor = find(&content, "a");

    assert!(!content.dispatch_event(anchor, "click", None));
}

#[test]
fn timers_run_when_time_advances() {
    let mut content = content_of(
        "<p id=t>old</p><script>setTimeout(function () { document.getElementById('t').textContent = 'new'; }, 50);</script>",
    );
    let due = content.next_timer_due().expect("a pending timer");

    content.advance_time(due + 1.0);

    assert!(content.is_stale());
    assert_eq!(content.next_timer_due(), None);
}

#[test]
fn content_without_scripts_has_no_timers() {
    let mut content = content_of("<p>hi</p>");

    assert_eq!(content.next_timer_due(), None);
    content.advance_time(1000.0);
    assert!(!content.is_stale());
}
