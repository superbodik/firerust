use fr_net::address::resolve_address::resolve_address;
use fr_net::load::request::Request;

pub fn request_for(input: &str) -> Option<Request> {
    let base_dir = std::env::current_dir().unwrap_or_default();
    let url = resolve_address(input, &base_dir).ok()?;
    Some(Request::get(url))
}
