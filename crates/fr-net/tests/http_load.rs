use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};

use fr_net::load::load_url::load_url;
use fr_net::load::net_error::NetError;
use url::Url;

fn serve_once(response: &'static str) -> (Url, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().expect("address").port();
    let handle = thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buffer = [0u8; 4096];
            let _ = stream.read(&mut buffer);
            let _ = stream.write_all(response.as_bytes());
        }
    });
    let url = Url::parse(&format!("http://127.0.0.1:{port}/")).expect("url");
    (url, handle)
}

#[test]
fn a_successful_response_is_read() {
    let (url, server) = serve_once(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 11\r\nConnection: close\r\n\r\nhello world",
    );

    let response = load_url(&url).expect("response");
    server.join().expect("server");

    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"hello world");
    assert_eq!(
        response.content_type.as_deref(),
        Some("text/html; charset=utf-8")
    );
}

#[test]
fn an_error_status_is_reported() {
    let (url, server) =
        serve_once("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");

    let result = load_url(&url);
    server.join().expect("server");

    assert!(matches!(result, Err(NetError::Status(404))));
}

#[test]
fn a_closed_port_is_reported() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().expect("address").port();
    drop(listener);
    let url = Url::parse(&format!("http://127.0.0.1:{port}/")).expect("url");

    let result = load_url(&url);

    assert!(matches!(result, Err(NetError::Http(_))));
}
