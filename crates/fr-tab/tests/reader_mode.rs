use fr_css::value::display::Display;
use fr_page::load::build_page::build_page;
use fr_tab::model::tab::Tab;
use fr_tab::model::tab_content::TabContent;
use fr_tab::model::tab_id::TabId;
use url::Url;

fn address() -> Url {
    Url::parse("https://example.test/article").unwrap()
}

fn page(title: &str, body: &str) -> fr_page::load::page::Page {
    build_page(
        &format!("<html><head><title>{title}</title></head><body>{body}</body></html>"),
        address(),
    )
}

fn tab_showing(title: &str, body: &str) -> Tab {
    Tab::from_page(TabId(1), page(title, body), 0)
}

#[test]
fn entering_the_reader_swaps_the_content_and_leaving_brings_it_back() {
    let mut tab = tab_showing("Original", "<p id=a>original body</p>");

    assert!(!tab.is_reading());
    assert!(tab.enter_reader(page("Reader", "<p id=b>reader body</p>")));

    assert!(tab.is_reading());
    assert_eq!(tab.title(), "Reader");
    let reading = tab.content().expect("content");
    assert!(reading.document.borrow().node_count() > 0);

    assert!(tab.leave_reader());
    assert!(!tab.is_reading());
    assert!(tab.content().is_some());
}

#[test]
fn leaving_without_having_entered_does_nothing() {
    let mut tab = tab_showing("Original", "<p>text</p>");

    assert!(!tab.leave_reader());
    assert!(tab.content().is_some());
}

#[test]
fn a_discarded_tab_cannot_enter_the_reader() {
    let mut tab = tab_showing("Original", "<p>text</p>");
    tab.discard();

    assert!(!tab.enter_reader(page("Reader", "<p>reader</p>")));
}

#[test]
fn navigating_or_discarding_forgets_the_original() {
    let mut tab = tab_showing("Original", "<p>text</p>");
    tab.enter_reader(page("Reader", "<p>reader</p>"));
    tab.show(page("Next", "<p>next</p>"));
    assert!(!tab.is_reading());

    tab.enter_reader(page("Reader", "<p>reader</p>"));
    tab.discard();
    assert!(!tab.is_reading());
}

#[test]
fn plain_styles_drop_the_page_own_rules_but_keep_the_browser_defaults() {
    let styled = page(
        "Styled",
        "<style>div { display: flex } p { color: #ff0000 }</style><div id=d><p id=p>x</p></div>",
    );
    let mut content = TabContent::from_page(styled);

    content.apply_plain_styles();

    assert!(content.is_plain);
    let document = content.document.borrow();
    let div = document
        .walk(document.root())
        .find_map(|edge| match edge {
            fr_dom::tree::edge::Edge::Enter(id)
                if document
                    .node(id)
                    .and_then(|node| node.kind().as_element())
                    .is_some_and(|element| element.attribute("id") == Some("d")) =>
            {
                Some(id)
            }
            _ => None,
        })
        .expect("div");
    assert_eq!(
        content.styles.get(div).expect("style").display,
        Display::Block
    );
}
