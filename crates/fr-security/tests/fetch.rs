use fr_security::fetch::block_reason::BlockReason;
use fr_security::fetch::check_fetch::check_fetch;
use fr_security::navigation::check_navigation::check_navigation;
use fr_security::origin::same_origin::same_origin;
use url::Url;

fn url(text: &str) -> Url {
    Url::parse(text).unwrap()
}

#[test]
fn a_secure_page_may_load_secure_resources() {
    assert_eq!(
        check_fetch(&url("https://a.test/"), &url("https://b.test/x.png")),
        Ok(())
    );
}

#[test]
fn a_secure_page_may_not_load_insecure_resources() {
    assert_eq!(
        check_fetch(&url("https://a.test/"), &url("http://b.test/x.png")),
        Err(BlockReason::MixedContent)
    );
}

#[test]
fn a_secure_page_may_load_from_the_local_machine() {
    assert_eq!(
        check_fetch(&url("https://a.test/"), &url("http://127.0.0.1:8080/x.png")),
        Ok(())
    );
    assert_eq!(
        check_fetch(&url("https://a.test/"), &url("http://localhost/x.png")),
        Ok(())
    );
}

#[test]
fn an_insecure_page_may_load_insecure_resources() {
    assert_eq!(
        check_fetch(&url("http://a.test/"), &url("http://b.test/x.css")),
        Ok(())
    );
}

#[test]
fn a_web_page_may_not_read_local_files() {
    assert_eq!(
        check_fetch(&url("http://a.test/"), &url("file:///C:/secret.txt")),
        Err(BlockReason::LocalFileFromWeb)
    );
    assert_eq!(
        check_fetch(&url("https://a.test/"), &url("file:///C:/secret.txt")),
        Err(BlockReason::LocalFileFromWeb)
    );
}

#[test]
fn a_local_page_may_load_local_files_and_the_web() {
    assert_eq!(
        check_fetch(
            &url("file:///C:/site/index.html"),
            &url("file:///C:/site/a.css")
        ),
        Ok(())
    );
    assert_eq!(
        check_fetch(
            &url("file:///C:/site/index.html"),
            &url("https://b.test/a.css")
        ),
        Ok(())
    );
}

#[test]
fn other_schemes_are_refused() {
    assert_eq!(
        check_fetch(&url("http://a.test/"), &url("ftp://b.test/x")),
        Err(BlockReason::UnsupportedScheme)
    );
    assert_eq!(
        check_fetch(&url("http://a.test/"), &url("javascript:alert(1)")),
        Err(BlockReason::UnsupportedScheme)
    );
}

#[test]
fn a_web_page_may_not_link_to_a_local_file() {
    assert_eq!(
        check_navigation(&url("https://a.test/"), &url("file:///C:/x.html")),
        Err(BlockReason::LocalFileFromWeb)
    );
}

#[test]
fn links_between_web_pages_and_from_local_pages_are_allowed() {
    assert_eq!(
        check_navigation(&url("https://a.test/"), &url("http://b.test/")),
        Ok(())
    );
    assert_eq!(
        check_navigation(&url("file:///C:/a.html"), &url("file:///C:/b.html")),
        Ok(())
    );
}

#[test]
fn only_identical_scheme_host_and_port_are_the_same_origin() {
    assert!(same_origin(
        &url("https://a.test/x"),
        &url("https://a.test/y?q")
    ));
    assert!(!same_origin(
        &url("https://a.test/"),
        &url("http://a.test/")
    ));
    assert!(!same_origin(
        &url("https://a.test/"),
        &url("https://b.test/")
    ));
    assert!(!same_origin(
        &url("https://a.test/"),
        &url("https://a.test:8443/")
    ));
    assert!(!same_origin(&url("file:///a.html"), &url("file:///a.html")));
}
