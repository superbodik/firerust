use fr_dom::node::node_id::NodeId;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::control::control_focus::ControlFocus;
use crate::control::control_kind::ControlKind;
use crate::control::control_label::ControlLabel;

pub fn label_of(
    document: &Document,
    node: NodeId,
    kind: ControlKind,
    focus: Option<ControlFocus>,
) -> ControlLabel {
    let attribute = |name: &str| attribute_of(document, node, name);
    let caret = focus.filter(|focus| focus.node == node).map(|f| f.caret);
    match kind {
        ControlKind::Text => {
            let value = attribute("value").unwrap_or_default();
            if value.is_empty()
                && caret.is_none()
                && let Some(placeholder) = attribute("placeholder")
            {
                return ControlLabel {
                    text: placeholder,
                    is_placeholder: true,
                };
            }
            plain(with_caret(&value, caret))
        }
        ControlKind::Password => {
            let masked = "•".repeat(attribute("value").unwrap_or_default().chars().count());
            plain(with_caret(&masked, caret))
        }
        ControlKind::TextArea => {
            let value = text_content(document, node);
            if value.is_empty()
                && caret.is_none()
                && let Some(placeholder) = attribute("placeholder")
            {
                return ControlLabel {
                    text: placeholder,
                    is_placeholder: true,
                };
            }
            plain(with_caret(&value, caret))
        }
        ControlKind::Button => plain(button_text(document, node)),
        ControlKind::Select => plain(format!("{} ▾", selected_option_text(document, node))),
        ControlKind::Checkbox => plain(mark(attribute("checked").is_some(), "✓")),
        ControlKind::Radio => plain(mark(attribute("checked").is_some(), "●")),
    }
}

fn plain(text: String) -> ControlLabel {
    ControlLabel {
        text,
        is_placeholder: false,
    }
}

fn mark(is_checked: bool, glyph: &str) -> String {
    if is_checked {
        glyph.to_owned()
    } else {
        String::new()
    }
}

fn with_caret(text: &str, caret: Option<usize>) -> String {
    let Some(caret) = caret else {
        return text.to_owned();
    };
    let mut shown = String::new();
    for (index, character) in text.chars().enumerate() {
        if index == caret {
            shown.push('|');
        }
        shown.push(character);
    }
    if caret >= text.chars().count() {
        shown.push('|');
    }
    shown
}

fn attribute_of(document: &Document, node: NodeId, name: &str) -> Option<String> {
    document
        .node(node)?
        .kind()
        .as_element()?
        .attribute(name)
        .map(str::to_owned)
}

fn button_text(document: &Document, node: NodeId) -> String {
    let element_name = document
        .node(node)
        .and_then(|found| found.kind().as_element())
        .map(|element| element.name().to_owned())
        .unwrap_or_default();
    if element_name == "button" {
        return text_content(document, node)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
    }
    if let Some(value) = attribute_of(document, node, "value") {
        return value;
    }
    match attribute_of(document, node, "type")
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "submit" => "Submit".to_owned(),
        "reset" => "Reset".to_owned(),
        _ => String::new(),
    }
}

fn selected_option_text(document: &Document, node: NodeId) -> String {
    let options: Vec<NodeId> = document
        .walk(node)
        .filter_map(|edge| match edge {
            Edge::Enter(id) if is_option(document, id) => Some(id),
            _ => None,
        })
        .collect();
    let chosen = options
        .iter()
        .copied()
        .find(|&id| attribute_of(document, id, "selected").is_some())
        .or_else(|| options.first().copied());
    chosen.map_or_else(String::new, |id| {
        text_content(document, id)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    })
}

fn is_option(document: &Document, id: NodeId) -> bool {
    document
        .node(id)
        .and_then(|node| node.kind().as_element())
        .is_some_and(|element| element.name() == "option")
}
