use std::io::Cursor;

use image::ImageReader;

use crate::decode::decoded_image::DecodedImage;
use crate::decode::premultiply::premultiply;

pub const MAX_PIXELS: u64 = 16_000_000;

pub fn decode_image(bytes: &[u8]) -> Option<DecodedImage> {
    let (width, height) = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()?
        .into_dimensions()
        .ok()?;
    if width == 0 || height == 0 || u64::from(width) * u64::from(height) > MAX_PIXELS {
        return None;
    }
    let decoded = image::load_from_memory(bytes).ok()?;
    let mut rgba = decoded.to_rgba8().into_raw();
    premultiply(&mut rgba);
    Some(DecodedImage::new(width, height, rgba))
}
