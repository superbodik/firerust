use fr_dom::query::find_anchor::find_anchor;
use fr_dom::tree::document::Document;

use crate::anchor::first_box_tops::first_box_tops;
use crate::boxes::layout_box::LayoutBox;

pub fn anchor_top(root: &LayoutBox, document: &Document, fragment: &str) -> Option<f32> {
    let target = find_anchor(document, fragment)?;
    let tops = first_box_tops(root);
    let mut current = Some(target);
    while let Some(id) = current {
        if let Some(top) = tops.get(&id) {
            return Some(*top);
        }
        current = document.node(id)?.parent();
    }
    None
}
