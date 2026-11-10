use fr_page::load::blank_page::{BLANK_ADDRESS, blank_page};

#[test]
fn the_blank_page_has_a_title_and_the_blank_address() {
    let page = blank_page().expect("blank page");

    assert_eq!(page.title.as_deref(), Some("New tab"));
    assert_eq!(page.url.as_str(), BLANK_ADDRESS);
}
