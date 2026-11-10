use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn attr_length(document: &Document, node: NodeId, name: &str) -> Option<Length> {
    let text = document
        .node(node)?
        .kind()
        .as_element()?
        .attribute(name)?
        .trim();
    if let Some(percent) = text.strip_suffix('%') {
        let value: f32 = percent.trim().parse().ok()?;
        return (value >= 0.0).then_some(Length::Percent(value));
    }
    let digits = text.strip_suffix("px").unwrap_or(text).trim();
    let value: f32 = digits.parse().ok()?;
    (value >= 0.0).then_some(Length::Px(value))
}
