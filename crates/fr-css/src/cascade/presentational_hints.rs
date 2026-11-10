use fr_dom::node::element_data::ElementData;

use crate::parse::declaration::Declaration;
use crate::parse::parse_color_text::parse_color_text;

pub fn presentational_hints(element: &ElementData) -> Vec<Declaration> {
    let mut hints: Vec<Declaration> = Vec::new();
    if let Some(color) = element.attribute("bgcolor").and_then(parse_color_text) {
        hints.push(Declaration::BackgroundColor(color));
    }
    if element.name() == "font"
        && let Some(color) = element.attribute("color").and_then(parse_color_text)
    {
        hints.push(Declaration::Color(color));
    }
    hints
}
