use fr_dom::error::dom_error::DomError;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::state::attribute_of::attribute_of;

pub fn toggle_checkbox(document: &mut Document, checkbox: NodeId) -> Result<bool, DomError> {
    if attribute_of(document, checkbox, "checked").is_some() {
        document.remove_attribute(checkbox, "checked")?;
        Ok(false)
    } else {
        document.set_attribute(checkbox, "checked", "")?;
        Ok(true)
    }
}
