use crate::import::parse_csv::parse_csv;
use crate::model::credential::Credential;
use crate::model::origin_of::origin_of;

const ADDRESS_NAMES: [&str; 5] = ["url", "origin", "hostname", "login_uri", "website"];
const USERNAME_NAMES: [&str; 4] = ["username", "login_username", "user", "login"];
const PASSWORD_NAMES: [&str; 2] = ["password", "login_password"];

pub fn import_csv(text: &str) -> Option<Vec<Credential>> {
    let rows = parse_csv(text);
    let header: Vec<String> = rows
        .first()?
        .iter()
        .map(|name| name.trim().to_ascii_lowercase())
        .collect();
    let address = column(&header, &ADDRESS_NAMES)?;
    let password = column(&header, &PASSWORD_NAMES)?;
    let username = column(&header, &USERNAME_NAMES);
    let mut found = Vec::new();
    for row in rows.iter().skip(1) {
        let cell = |index: usize| row.get(index).map(String::as_str).unwrap_or("");
        let secret = cell(password);
        let Some(origin) = origin_of(cell(address)) else {
            continue;
        };
        if secret.is_empty() {
            continue;
        }
        let name = username.map(cell).unwrap_or("");
        found.push(Credential::new(&origin, name, secret));
    }
    Some(found)
}

fn column(header: &[String], names: &[&str]) -> Option<usize> {
    names
        .iter()
        .find_map(|wanted| header.iter().position(|name| name == wanted))
}
