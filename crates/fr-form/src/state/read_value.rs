use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;

use crate::state::attribute_of::attribute_of;

pub fn read_value(document: &Document, control: NodeId) -> String {
    if element_name(document, control) == Some("textarea") {
        return text_content(document, control);
    }
    attribute_of(document, control, "value").unwrap_or_default()
}
