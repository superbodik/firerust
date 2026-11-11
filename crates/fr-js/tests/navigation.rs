mod support;

use fr_js::natives::navigate_request::NavigationRequest;
use support::Page;

fn request(address: &str, in_new_tab: bool) -> Option<NavigationRequest> {
    Some(NavigationRequest {
        address: address.to_owned(),
        in_new_tab,
    })
}

#[test]
fn assigning_the_href_asks_the_browser_to_navigate() {
    let mut page = Page::new("<p></p>");

    page.run("location.href = '/next?x=1'");

    assert_eq!(page.runtime.take_navigation(), request("/next?x=1", false));
}

#[test]
fn assign_and_replace_navigate_too() {
    let mut page = Page::new("<p></p>");

    page.run("location.assign('/a')");
    assert_eq!(page.runtime.take_navigation(), request("/a", false));

    page.run("location.replace('https://other.test/b')");
    assert_eq!(
        page.runtime.take_navigation(),
        request("https://other.test/b", false)
    );
}

#[test]
fn reload_asks_for_the_current_address() {
    let mut page = Page::new("<p></p>");

    page.run("location.reload()");

    assert_eq!(
        page.runtime.take_navigation(),
        request("https://example.test/page", false)
    );
}

#[test]
fn window_open_asks_for_a_new_tab_and_returns_null() {
    let mut page = Page::new("<p></p>");

    page.run("console.log(window.open('/popup') === null)");

    assert_eq!(page.logs(), vec!["[log] true"]);
    assert_eq!(page.runtime.take_navigation(), request("/popup", true));
}

#[test]
fn only_the_last_request_of_a_script_is_kept() {
    let mut page = Page::new("<p></p>");

    page.run("location.href = '/first'; location.href = '/second'");

    assert_eq!(page.runtime.take_navigation(), request("/second", false));
    assert_eq!(page.runtime.take_navigation(), None);
}

#[test]
fn a_script_without_navigation_leaves_nothing_behind() {
    let mut page = Page::new("<p></p>");

    page.run("var x = location.href");

    assert_eq!(page.runtime.take_navigation(), None);
}

#[test]
fn the_parts_of_the_address_are_available() {
    let mut page = Page::new("<p></p>");

    page.run(
        "console.log(location.protocol, location.hostname, location.port, \
         location.pathname, location.search, location.hash, location.origin, location.host)",
    );

    assert_eq!(
        page.logs(),
        vec!["[log] https: example.test  /page   https://example.test example.test"]
    );
}

#[test]
fn the_href_reads_back_and_prints_as_the_address() {
    let mut page = Page::new("<p></p>");

    page.run("console.log(location.href, String(location), '' + location)");

    assert_eq!(
        page.logs(),
        vec!["[log] https://example.test/page https://example.test/page https://example.test/page"]
    );
}
