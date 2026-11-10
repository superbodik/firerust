use serde_json::{Value, json};

use crate::model::password_list::PasswordList;

pub fn encode_vault(list: &PasswordList) -> Vec<u8> {
    let items: Vec<Value> = list
        .items()
        .iter()
        .map(|credential| {
            json!({
                "origin": credential.origin,
                "username": credential.username,
                "password": credential.password,
            })
        })
        .collect();
    let document = json!({ "version": 1, "logins": items });
    serde_json::to_vec(&document).unwrap_or_default()
}
