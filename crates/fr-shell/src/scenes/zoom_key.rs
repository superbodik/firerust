use url::Url;

pub fn zoom_key(url: &Url) -> String {
    match url.host_str() {
        Some(host) if !host.is_empty() => host.to_ascii_lowercase(),
        _ => url.scheme().to_owned(),
    }
}
