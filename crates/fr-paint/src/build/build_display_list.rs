use fr_layout::boxes::layout_box::LayoutBox;

use crate::build::box_items::box_items;
use crate::display::display_list::DisplayList;

pub fn build_display_list(root: &LayoutBox) -> DisplayList {
    let mut items = Vec::new();
    let mut pending: Vec<&LayoutBox> = vec![root];
    while let Some(layout_box) = pending.pop() {
        box_items(layout_box, &mut items);
        pending.extend(layout_box.children.iter().rev());
    }
    DisplayList { items }
}
