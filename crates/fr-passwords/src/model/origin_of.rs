use url::Url;

pub fn origin_of(address: &str) -> Option<String> {
    let url = Url::parse(address.trim()).ok()?;
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }
    url.host_str()?;
    Some(url.origin().ascii_serialization())
}
