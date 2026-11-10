use fr_dom::tree::document::Document;

use crate::login::login_fields::LoginFields;
use crate::state::read_value::read_value;
use crate::state::write_value::write_value;

pub fn fill_login(
    document: &mut Document,
    fields: &LoginFields,
    username: &str,
    password: &str,
) -> bool {
    if !read_value(document, fields.password).is_empty() {
        return false;
    }
    if let Some(field) = fields.username {
        if !read_value(document, field).is_empty() {
            return false;
        }
        if write_value(document, field, username).is_err() {
            return false;
        }
    }
    write_value(document, fields.password, password).is_ok()
}
