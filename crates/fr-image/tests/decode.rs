use fr_image::decode::decode_image::decode_image;

const PNG: &[u8] = include_bytes!("../../../tests/pages/gradient.png");
const JPEG: &[u8] = include_bytes!("../../../tests/pages/photo.jpg");
const GIF: &[u8] = include_bytes!("../../../tests/pages/badge.gif");

fn pixel(image: &fr_image::decode::decoded_image::DecodedImage, x: u32, y: u32) -> [u8; 4] {
    let start = ((y * image.width() + x) * 4) as usize;
    let data = image.premultiplied_rgba();
    [
        data[start],
        data[start + 1],
        data[start + 2],
        data[start + 3],
    ]
}

#[test]
fn a_png_is_decoded_with_its_size() {
    let image = decode_image(PNG).expect("png");

    assert_eq!((image.width(), image.height()), (240, 120));
    assert_eq!(
        image.premultiplied_rgba().len(),
        (image.width() * image.height() * 4) as usize
    );
}

#[test]
fn a_transparent_png_pixel_is_fully_transparent_and_premultiplied() {
    let image = decode_image(PNG).expect("png");

    assert_eq!(pixel(&image, 120, 60), [0, 0, 0, 0]);
    assert_eq!(pixel(&image, 0, 0), [0, 0, 0, 0]);
    assert_eq!(pixel(&image, 30, 30)[3], 255);
}

#[test]
fn a_jpeg_is_decoded_as_opaque() {
    let image = decode_image(JPEG).expect("jpeg");

    assert_eq!((image.width(), image.height()), (320, 180));
    assert_eq!(pixel(&image, 160, 90)[3], 255);
}

#[test]
fn a_gif_is_decoded() {
    let image = decode_image(GIF).expect("gif");

    assert_eq!((image.width(), image.height()), (64, 64));
}

#[test]
fn garbage_is_not_an_image() {
    assert!(decode_image(b"this is not an image").is_none());
    assert!(decode_image(&[]).is_none());
    assert!(decode_image(&PNG[..40]).is_none());
}
