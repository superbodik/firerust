use url::Url;

const MAIL_FIELDS: [&str; 4] = ["subject", "body", "cc", "bcc"];
const PHONE_CHARACTERS: &str = "+-.() *#;,0123456789";

pub fn external_url(url: &Url) -> Option<Url> {
    match url.scheme() {
        "mailto" => mail_url(url),
        "tel" => phone_url(url),
        _ => None,
    }
}

fn mail_url(url: &Url) -> Option<Url> {
    let mut safe = Url::parse("mailto:placeholder").ok()?;
    safe.set_path(url.path());
    let fields: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(key, _)| MAIL_FIELDS.contains(&key.to_ascii_lowercase().as_str()))
        .map(|(key, value)| (key.to_ascii_lowercase(), value.into_owned()))
        .collect();
    if !fields.is_empty() {
        safe.query_pairs_mut().extend_pairs(fields);
    }
    Some(safe)
}

fn phone_url(url: &Url) -> Option<Url> {
    let number = url.path();
    let is_clean = !number.is_empty()
        && number
            .chars()
            .all(|character| PHONE_CHARACTERS.contains(character));
    is_clean.then(|| url.clone())
}
