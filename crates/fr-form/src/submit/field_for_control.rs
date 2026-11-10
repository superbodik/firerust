use fr_dom::node::node_id::NodeId;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_layout::control::control_kind::ControlKind;
use fr_layout::control::control_kind_of::control_kind_of;

use crate::state::attribute_of::attribute_of;
use crate::submit::form_field::FormField;

pub fn field_for_control(
    document: &Document,
    control: NodeId,
    submitter: Option<NodeId>,
) -> Option<FormField> {
    let kind = control_kind_of(document, control);
    let name = attribute_of(document, control, "name").filter(|name| !name.is_empty())?;
    if attribute_of(document, control, "disabled").is_some() {
        return None;
    }
    let element = document.node(control)?.kind().as_element()?;
    match (element.name(), kind) {
        ("input", Some(ControlKind::Checkbox | ControlKind::Radio)) => {
            attribute_of(document, control, "checked")?;
            let value = attribute_of(document, control, "value").unwrap_or_else(|| "on".to_owned());
            Some(FormField { name, value })
        }
        ("input", Some(ControlKind::Button)) | ("button", _) => {
            if submitter != Some(control) {
                return None;
            }
            let value = attribute_of(document, control, "value").unwrap_or_default();
            Some(FormField { name, value })
        }
        ("textarea", _) => Some(FormField {
            name,
            value: text_content(document, control),
        }),
        ("select", _) => Some(FormField {
            name,
            value: selected_value(document, control),
        }),
        ("input", _) => {
            let value = attribute_of(document, control, "value").unwrap_or_default();
            Some(FormField { name, value })
        }
        _ => None,
    }
}

fn selected_value(document: &Document, select: NodeId) -> String {
    let options: Vec<NodeId> = document
        .walk(select)
        .filter_map(|edge| match edge {
            Edge::Enter(id)
                if document
                    .node(id)
                    .and_then(|node| node.kind().as_element())
                    .is_some_and(|element| element.name() == "option") =>
            {
                Some(id)
            }
            _ => None,
        })
        .collect();
    let chosen = options
        .iter()
        .copied()
        .find(|&id| attribute_of(document, id, "selected").is_some())
        .or_else(|| options.first().copied());
    let Some(option) = chosen else {
        return String::new();
    };
    attribute_of(document, option, "value").unwrap_or_else(|| {
        text_content(document, option)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    })
}
