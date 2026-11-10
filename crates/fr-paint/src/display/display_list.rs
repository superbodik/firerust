use crate::display::display_item::DisplayItem;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayList {
    pub items: Vec<DisplayItem>,
}
