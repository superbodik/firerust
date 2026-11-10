use std::sync::Arc;

use fr_image::decode::decoded_image::DecodedImage;

use crate::boxes::text_box::TextBox;

#[derive(Debug, Clone, PartialEq)]
pub enum BoxKind {
    Block,
    Text(TextBox),
    Marker(TextBox),
    Image(Arc<DecodedImage>),
}
