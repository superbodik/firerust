use crate::routes::decode_percent::decode_percent;

pub fn parse_form(text: &str) -> Vec<(String, String)> {
    text.split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            (decode_percent(name), decode_percent(value))
        })
        .collect()
}
