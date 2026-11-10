use fr_geom::color::rgba::Rgba;
use fr_paint::display::display_list::DisplayList;

pub struct CachedLayout {
    pub width: u32,
    pub background: Rgba,
    pub list: DisplayList,
}
