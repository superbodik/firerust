use fr_net::address::classify_input::classify_input;
use fr_net::address::input_kind::InputKind;
use fr_net::address::resolve_address::resolve_address;
use fr_net::load::request::Request;
use fr_settings::model::search_engine::SearchEngine;
use url::Url;

pub fn request_for(input: &str, engine: SearchEngine) -> Option<Request> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    let base_dir = std::env::current_dir().unwrap_or_default();
    if classify_input(trimmed, &base_dir) == InputKind::Search {
        let url = Url::parse(&engine.query_url(trimmed)).ok()?;
        return Some(Request::get(url));
    }
    if let Ok(url) = Url::parse(trimmed)
        && url.scheme() == "about"
    {
        return Some(Request::get(url));
    }
    let url = resolve_address(trimmed, &base_dir).ok()?;
    Some(Request::get(url))
}
