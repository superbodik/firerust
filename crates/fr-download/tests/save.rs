use std::fs;
use std::path::PathBuf;

use fr_download::save::save_download::save_download;
use fr_net::load::response::Response;
use url::Url;

fn scratch_folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "firerust-download-test-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&folder);
    folder
}

fn file_response(name: &str, body: &[u8]) -> Response {
    Response {
        url: Url::parse(&format!("https://example.com/{name}")).expect("address"),
        status: 200,
        content_type: Some("application/octet-stream".to_owned()),
        content_disposition: None,
        body: body.to_vec(),
    }
}

#[test]
fn the_file_is_written_into_the_folder() {
    let folder = scratch_folder("write");

    let path = save_download(&file_response("data.bin", b"hello"), &folder).expect("saved");

    assert_eq!(path, folder.join("data.bin"));
    assert_eq!(fs::read(&path).expect("read"), b"hello");
    let _ = fs::remove_dir_all(&folder);
}

#[test]
fn a_second_download_with_the_same_name_gets_a_number() {
    let folder = scratch_folder("number");

    let first = save_download(&file_response("data.bin", b"one"), &folder).expect("first");
    let second = save_download(&file_response("data.bin", b"two"), &folder).expect("second");
    let third = save_download(&file_response("data.bin", b"three"), &folder).expect("third");

    assert_eq!(first, folder.join("data.bin"));
    assert_eq!(second, folder.join("data (1).bin"));
    assert_eq!(third, folder.join("data (2).bin"));
    assert_eq!(fs::read(&first).expect("read"), b"one");
    assert_eq!(fs::read(&second).expect("read"), b"two");
    let _ = fs::remove_dir_all(&folder);
}

#[test]
fn a_missing_folder_is_created() {
    let folder = scratch_folder("create").join("nested");

    let path = save_download(&file_response("x.txt", b"x"), &folder).expect("saved");

    assert!(path.is_file());
    let _ = fs::remove_dir_all(folder.parent().expect("parent"));
}

#[test]
fn a_hostile_name_cannot_leave_the_folder() {
    let folder = scratch_folder("escape");
    let mut response = file_response("x", b"payload");
    response.content_disposition = Some("attachment; filename=\"..\\..\\escaped.txt\"".to_owned());

    let path = save_download(&response, &folder).expect("saved");

    assert_eq!(path.parent(), Some(folder.as_path()));
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("escaped.txt")
    );
    let _ = fs::remove_dir_all(&folder);
}
