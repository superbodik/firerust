use std::sync::Arc;

use fr_image::decode::decode_image::decode_image;
use fr_image::decode::decoded_image::DecodedImage;
use fr_net::load::load_url::load_url;
use url::Url;

pub fn load_image(url: &Url) -> Option<Arc<DecodedImage>> {
    let response = load_url(url).ok()?;
    decode_image(&response.body).map(Arc::new)
}
