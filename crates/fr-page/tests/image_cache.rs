use std::path::PathBuf;
use std::sync::Arc;

use fr_image::decode::decoded_image::DecodedImage;
use fr_page::images::load_image::load_image;
use fr_page::images::shared_image_cache::{
    IMAGE_CACHE_BYTES, cached_image, remember_image, shared_image_bytes,
};
use url::Url;

fn picture_url(name: &str) -> Url {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/pages")
        .join(name)
        .canonicalize()
        .expect("test picture");
    Url::from_file_path(path).expect("file url")
}

#[test]
fn loading_the_same_picture_twice_shares_one_decoded_copy() {
    let url = picture_url("gradient.png");

    let first = load_image(&url).expect("first load");
    let second = load_image(&url).expect("second load");

    assert!(Arc::ptr_eq(&first, &second));
}

#[test]
fn a_remembered_image_can_be_found_by_address() {
    let image = Arc::new(DecodedImage::new(1, 1, vec![0, 0, 0, 255]));

    remember_image("test://remembered", &image);

    let found = cached_image("test://remembered").expect("cached");
    assert!(Arc::ptr_eq(&image, &found));
}

#[test]
fn an_unknown_address_is_not_cached() {
    assert!(cached_image("test://never-stored").is_none());
}

#[test]
fn the_cache_stays_under_its_limit() {
    let url = picture_url("photo.jpg");
    load_image(&url).expect("load");

    assert!(shared_image_bytes() <= IMAGE_CACHE_BYTES);
    assert!(shared_image_bytes() > 0);
}
