use fr_net::address::resolve_address::resolve_address;
use fr_net::load::request::Request;
use url::Url;

pub fn request_for(input: &str) -> Option<Request> {
    let trimmed = input.trim();
    if let Ok(url) = Url::parse(trimmed)
        && url.scheme() == "about"
    {
        return Some(Request::get(url));
    }
    let base_dir = std::env::current_dir().unwrap_or_default();
    let url = resolve_address(trimmed, &base_dir).ok()?;
    Some(Request::get(url))
}
