use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::control::control_kind::ControlKind;

pub fn control_kind_of(document: &Document, node: NodeId) -> Option<ControlKind> {
    let element = document.node(node)?.kind().as_element()?;
    match element.name() {
        "textarea" => Some(ControlKind::TextArea),
        "select" => Some(ControlKind::Select),
        "button" => Some(ControlKind::Button),
        "input" => {
            let declared = element.attribute("type").unwrap_or("text");
            match declared.trim().to_ascii_lowercase().as_str() {
                "hidden" => None,
                "password" => Some(ControlKind::Password),
                "checkbox" => Some(ControlKind::Checkbox),
                "radio" => Some(ControlKind::Radio),
                "submit" | "reset" | "button" | "image" => Some(ControlKind::Button),
                _ => Some(ControlKind::Text),
            }
        }
        _ => None,
    }
}
