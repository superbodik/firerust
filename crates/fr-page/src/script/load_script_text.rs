use fr_net::load::load_url::load_url;
use url::Url;

pub fn load_script_text(url: &Url) -> Option<String> {
    let response = load_url(url).ok()?;
    if response.status >= 400 {
        return None;
    }
    Some(String::from_utf8_lossy(&response.body).into_owned())
}
