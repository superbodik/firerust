use fr_dom::error::dom_error::DomError;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;

pub fn write_value(document: &mut Document, control: NodeId, value: &str) -> Result<(), DomError> {
    if element_name(document, control) == Some("textarea") {
        return document.set_text(control, value);
    }
    document.set_attribute(control, "value", value)
}
