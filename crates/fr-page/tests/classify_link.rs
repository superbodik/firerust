use fr_page::link::classify_link::classify_link;
use fr_page::link::link_action::LinkAction;
use url::Url;

fn base() -> Url {
    Url::parse("https://example.com/docs/page.html?x=1").unwrap()
}

fn open(address: &str, in_new_tab: bool) -> Option<LinkAction> {
    Some(LinkAction::Open {
        url: Url::parse(address).unwrap(),
        in_new_tab,
    })
}

fn scroll(fragment: &str) -> Option<LinkAction> {
    Some(LinkAction::Scroll {
        fragment: fragment.to_owned(),
    })
}

#[test]
fn an_ordinary_link_opens_in_the_same_tab() {
    assert_eq!(
        classify_link(&base(), "next.html", None),
        open("https://example.com/docs/next.html", false)
    );
}

#[test]
fn target_blank_opens_a_new_tab_and_other_targets_do_not() {
    assert_eq!(
        classify_link(&base(), "next.html", Some("_blank")),
        open("https://example.com/docs/next.html", true)
    );
    assert_eq!(
        classify_link(&base(), "next.html", Some(" _BLANK ")),
        open("https://example.com/docs/next.html", true)
    );
    assert_eq!(
        classify_link(&base(), "next.html", Some("_self")),
        open("https://example.com/docs/next.html", false)
    );
    assert_eq!(
        classify_link(&base(), "next.html", Some("sidebar")),
        open("https://example.com/docs/next.html", false)
    );
}

#[test]
fn a_fragment_link_scrolls() {
    assert_eq!(classify_link(&base(), "#intro", None), scroll("intro"));
    assert_eq!(classify_link(&base(), "#", None), scroll(""));
}

#[test]
fn a_fragment_is_percent_decoded() {
    assert_eq!(
        classify_link(&base(), "#%D1%80%D0%B0%D0%B7%D0%B4%D0%B5%D0%BB", None),
        scroll("раздел")
    );
}

#[test]
fn the_page_own_address_with_a_fragment_scrolls_instead_of_reloading() {
    assert_eq!(
        classify_link(&base(), "page.html?x=1#deep", None),
        scroll("deep")
    );
    assert_eq!(
        classify_link(&base(), "https://example.com/docs/page.html?x=1#deep", None),
        scroll("deep")
    );
}

#[test]
fn the_same_path_with_another_query_is_a_new_page() {
    assert_eq!(
        classify_link(&base(), "page.html?x=2#deep", None),
        open("https://example.com/docs/page.html?x=2#deep", false)
    );
}

#[test]
fn a_mailto_link_is_handed_to_the_system() {
    let action = classify_link(&base(), "mailto:team@example.com?subject=Hi", None);

    assert_eq!(
        action,
        Some(LinkAction::External {
            url: Url::parse("mailto:team@example.com?subject=Hi").unwrap()
        })
    );
}

#[test]
fn a_mailto_link_loses_every_field_but_the_message_ones() {
    let action = classify_link(
        &base(),
        "mailto:a@example.com?attach=C:%5Csecret.txt&subject=Hello&body=Text&cc=b@example.com",
        None,
    );

    let Some(LinkAction::External { url }) = action else {
        panic!("expected an external link");
    };
    assert!(!url.as_str().contains("attach"));
    assert!(url.as_str().contains("subject=Hello"));
    assert!(url.as_str().contains("body=Text"));
    assert!(url.as_str().contains("cc=b%40example.com"));
}

#[test]
fn a_phone_link_is_allowed_only_with_a_plain_number() {
    assert!(matches!(
        classify_link(&base(), "tel:+1-555-0100", None),
        Some(LinkAction::External { .. })
    ));
    assert_eq!(classify_link(&base(), "tel:calc.exe", None), None);
}

#[test]
fn dangerous_schemes_are_refused() {
    assert_eq!(classify_link(&base(), "javascript:alert(1)", None), None);
    assert_eq!(
        classify_link(&base(), "data:text/html,<b>x</b>", None),
        None
    );
    assert_eq!(classify_link(&base(), "ftp://example.com/f", None), None);
    assert_eq!(
        classify_link(&base(), "file:///C:/Windows/win.ini", None),
        None
    );
}

#[test]
fn an_empty_link_is_ignored() {
    assert_eq!(classify_link(&base(), "   ", None), None);
}

#[test]
fn an_internal_page_can_link_to_another_internal_page_and_the_web() {
    let menu = Url::parse("about:menu").unwrap();

    assert_eq!(
        classify_link(&menu, "about:bookmarks", None),
        open("about:bookmarks", false)
    );
    assert_eq!(
        classify_link(&menu, "https://example.com/", None),
        open("https://example.com/", false)
    );
}

#[test]
fn hover_text_shows_the_destination() {
    let page = classify_link(&base(), "next.html", None).unwrap();
    let anchor = classify_link(&base(), "#top", None).unwrap();

    assert_eq!(page.hover_text(), "https://example.com/docs/next.html");
    assert_eq!(anchor.hover_text(), "#top");
}
