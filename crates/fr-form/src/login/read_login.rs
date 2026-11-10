use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::login::find_login_fields::find_login_fields;
use crate::state::read_value::read_value;

pub fn read_login(document: &Document, form: NodeId) -> Option<(String, String)> {
    let fields = find_login_fields(document)
        .into_iter()
        .find(|fields| fields.form == Some(form))?;
    let password = read_value(document, fields.password);
    if password.is_empty() {
        return None;
    }
    let username = fields
        .username
        .map(|field| read_value(document, field))
        .unwrap_or_default();
    Some((username, password))
}
