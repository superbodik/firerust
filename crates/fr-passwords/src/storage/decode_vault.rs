use serde_json::Value;

use crate::model::credential::Credential;
use crate::model::password_list::PasswordList;

pub fn decode_vault(bytes: &[u8]) -> Option<PasswordList> {
    let document: Value = serde_json::from_slice(bytes).ok()?;
    let items = document.get("logins")?.as_array()?;
    let mut list = PasswordList::new();
    for item in items {
        let field = |name: &str| item.get(name).and_then(Value::as_str).unwrap_or("");
        list.save(Credential::new(
            field("origin"),
            field("username"),
            field("password"),
        ));
    }
    Some(list)
}
