use fr_geom::color::rgba::Rgba;
use fr_layout::boxes::layout_box::LayoutBox;
use fr_paint::display::display_list::DisplayList;

#[derive(Debug, Clone)]
pub struct LayoutCache {
    pub width: u32,
    pub height: f32,
    pub background: Rgba,
    pub root: LayoutBox,
    pub list: DisplayList,
}
