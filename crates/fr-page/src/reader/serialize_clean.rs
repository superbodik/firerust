use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::load::escape_html::escape_html;
use crate::reader::has_noise_token::has_noise_token;
use crate::reader::is_safe_reference::is_safe_reference;
use crate::reader::link_density::link_density;
use crate::reader::tag_role::{TagRole, tag_role};
use crate::reader::text_length::text_length;

const MAX_OUTPUT: usize = 1_500_000;
const MAX_IMAGES: usize = 20;
const LINK_LIST_DENSITY: f32 = 0.65;
const LINK_LIST_LIMIT: usize = 1_200;

enum Step {
    Open(NodeId),
    Close(String),
}

pub fn serialize_clean(document: &Document, root: NodeId) -> String {
    let mut output = String::new();
    let mut images = 0usize;
    let mut stack = vec![Step::Open(root)];
    while let Some(step) = stack.pop() {
        if output.len() > MAX_OUTPUT {
            break;
        }
        match step {
            Step::Close(tag) => {
                output.push_str("</");
                output.push_str(&tag);
                output.push('>');
            }
            Step::Open(id) => open_node(document, id, root, &mut output, &mut stack, &mut images),
        }
    }
    output
}

fn open_node(
    document: &Document,
    id: NodeId,
    root: NodeId,
    output: &mut String,
    stack: &mut Vec<Step>,
    images: &mut usize,
) {
    let Some(node) = document.node(id) else {
        return;
    };
    if let Some(text) = node.kind().as_text() {
        output.push_str(&escape_html(text));
        return;
    }
    let Some(element) = node.kind().as_element() else {
        return;
    };
    let name = element.name();
    if id != root && is_noise(element.attribute("class"), element.attribute("id"), element) {
        return;
    }
    match tag_role(name) {
        TagRole::Drop => {}
        TagRole::Unwrap => push_children(document, id, stack),
        TagRole::Keep => keep_node(document, id, name, output, stack, images),
    }
}

fn is_noise(
    class: Option<&str>,
    id: Option<&str>,
    element: &fr_dom::node::element_data::ElementData,
) -> bool {
    has_noise_token(class, id)
        || element.attribute("hidden").is_some()
        || element
            .attribute("style")
            .is_some_and(|style| style.replace(' ', "").contains("display:none"))
}

fn keep_node(
    document: &Document,
    id: NodeId,
    name: &str,
    output: &mut String,
    stack: &mut Vec<Step>,
    images: &mut usize,
) {
    let attribute = |wanted: &str| {
        document
            .node(id)
            .and_then(|node| node.kind().as_element())
            .and_then(|element| element.attribute(wanted))
    };
    if is_link_list(name) && is_mostly_links(document, id) {
        return;
    }
    match name {
        "img" => {
            let Some(source) = attribute("src").filter(|value| is_safe_reference(value)) else {
                return;
            };
            if *images >= MAX_IMAGES {
                return;
            }
            *images += 1;
            output.push_str(&format!(
                "<img src=\"{}\" alt=\"{}\">",
                escape_html(source),
                escape_html(attribute("alt").unwrap_or(""))
            ));
        }
        "br" | "hr" => {
            output.push('<');
            output.push_str(name);
            output.push('>');
        }
        "a" => match attribute("href").filter(|value| is_safe_reference(value)) {
            Some(target) => {
                output.push_str(&format!("<a href=\"{}\">", escape_html(target)));
                stack.push(Step::Close("a".to_owned()));
                push_children(document, id, stack);
            }
            None => push_children(document, id, stack),
        },
        other => {
            output.push('<');
            output.push_str(other);
            output.push('>');
            stack.push(Step::Close(other.to_owned()));
            push_children(document, id, stack);
        }
    }
}

fn is_link_list(name: &str) -> bool {
    matches!(name, "ul" | "ol" | "table" | "dl")
}

fn is_mostly_links(document: &Document, id: NodeId) -> bool {
    text_length(document, id) < LINK_LIST_LIMIT && link_density(document, id) > LINK_LIST_DENSITY
}

fn push_children(document: &Document, id: NodeId, stack: &mut Vec<Step>) {
    let children: Vec<NodeId> = document.children(id).collect();
    for child in children.into_iter().rev() {
        stack.push(Step::Open(child));
    }
}
