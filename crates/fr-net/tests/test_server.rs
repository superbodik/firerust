use fr_net::body::decode_text::decode_text;
use fr_net::load::load_request::load_request;
use fr_net::load::load_url::load_url;
use fr_net::load::net_error::NetError;
use fr_net::load::request::Request;
use fr_test_server::server::test_server::TestServer;
use url::Url;

fn url(server: &TestServer, path: &str) -> Url {
    Url::parse(&server.url(path)).unwrap()
}

fn text(response: &fr_net::load::response::Response) -> String {
    decode_text(&response.body, response.content_type.as_deref())
}

#[test]
fn a_plain_page_is_loaded() {
    let server = TestServer::start().unwrap();

    let response = load_url(&url(&server, "/plain")).unwrap();

    assert_eq!(response.status, 200);
    assert!(text(&response).contains("Hello from the test server"));
}

#[test]
fn a_windows_1251_page_with_a_header_charset_is_decoded() {
    let server = TestServer::start().unwrap();

    let response = load_url(&url(&server, "/cp1251")).unwrap();

    assert!(text(&response).contains("Привет, мир"));
}

#[test]
fn a_windows_1251_page_with_a_meta_charset_is_decoded() {
    let server = TestServer::start().unwrap();

    let response = load_url(&url(&server, "/meta-cp1251")).unwrap();

    assert!(text(&response).contains("Привет, мир"));
}

#[test]
fn gzip_bodies_are_unpacked() {
    let server = TestServer::start().unwrap();

    let response = load_url(&url(&server, "/gzip")).unwrap();

    assert!(text(&response).contains("compressed with gzip"));
}

#[test]
fn brotli_bodies_are_unpacked() {
    let server = TestServer::start().unwrap();

    let response = load_url(&url(&server, "/brotli")).unwrap();

    assert!(text(&response).contains("compressed with brotli"));
}

#[test]
fn redirects_are_followed_and_the_final_address_is_reported() {
    let server = TestServer::start().unwrap();

    let response = load_url(&url(&server, "/redirect-twice")).unwrap();

    assert_eq!(response.url.path(), "/plain");
    assert!(text(&response).contains("Hello from the test server"));
}

#[test]
fn an_error_status_is_an_error_for_resources() {
    let server = TestServer::start().unwrap();

    let result = load_url(&url(&server, "/missing"));

    assert!(matches!(result, Err(NetError::Status(404))));
}

#[test]
fn an_error_status_keeps_its_body_for_pages() {
    let server = TestServer::start().unwrap();

    let response = load_request(&Request::get(url(&server, "/missing"))).unwrap();

    assert_eq!(response.status, 404);
    assert!(text(&response).contains("Not found"));
}

#[test]
fn cookies_set_by_the_server_come_back() {
    let server = TestServer::start().unwrap();

    load_url(&url(&server, "/set-cookie")).unwrap();
    let response = load_url(&url(&server, "/show-cookie")).unwrap();

    let body = text(&response);
    assert!(body.contains("theme=dark"), "cookie page was {body}");
    assert!(body.contains("visit=1"), "cookie page was {body}");
}

#[test]
fn a_cookie_limited_to_a_path_is_not_sent_elsewhere() {
    let server = TestServer::start().unwrap();
    load_url(&url(&server, "/set-cookie")).unwrap();

    let response = load_request(&Request::get(url(&server, "/echo"))).unwrap();

    assert!(!text(&response).contains("visit=1"));
}

#[test]
fn a_post_sends_its_body_and_content_type() {
    let server = TestServer::start().unwrap();

    let request = Request::post(
        url(&server, "/echo"),
        b"a=1&b=two".to_vec(),
        "application/x-www-form-urlencoded",
    );
    let response = load_request(&request).unwrap();

    let body = text(&response);
    assert!(body.contains(">POST<"));
    assert!(body.contains(">a=1&b=two<"));
    assert!(body.contains("application/x-www-form-urlencoded"));
}

#[test]
fn a_wrong_password_shows_the_error_page() {
    let server = TestServer::start().unwrap();

    let request = Request::post(
        url(&server, "/login"),
        b"user=ada&password=nope".to_vec(),
        "application/x-www-form-urlencoded",
    );
    let response = load_request(&request).unwrap();

    assert_eq!(response.status, 401);
    assert!(text(&response).contains("Wrong user or password"));
}

#[test]
fn logging_in_sets_a_session_that_the_next_request_uses() {
    let server = TestServer::start().unwrap();

    let request = Request::post(
        url(&server, "/login"),
        b"user=ada&password=lovelace".to_vec(),
        "application/x-www-form-urlencoded",
    );
    let landing = load_request(&request).unwrap();
    let account = load_url(&url(&server, "/account")).unwrap();

    assert_eq!(landing.url.path(), "/account");
    assert!(text(&landing).contains("Welcome back, ada"));
    assert!(text(&account).contains("Welcome back, ada"));
}
