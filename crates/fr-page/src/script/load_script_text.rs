use fr_net::body::decode_text::decode_text;
use fr_net::load::load_url::load_url;
use url::Url;

pub fn load_script_text(url: &Url) -> Option<String> {
    let response = load_url(url).ok()?;
    Some(decode_text(
        &response.body,
        response.content_type.as_deref(),
    ))
}
