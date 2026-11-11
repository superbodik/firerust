use fr_geom::geometry::rect::Rect;

use crate::menu::menu_entries::menu_entries;
use crate::menu::menu_entry::MenuEntry;
use crate::menu::menu_kind::MenuKind;
use crate::menu::menu_metrics::{PADDING, entry_height, menu_rect};

pub fn entry_rects(
    kind: MenuKind,
    viewport_width: f32,
    viewport_height: f32,
) -> Vec<(MenuEntry, Rect)> {
    let popup = menu_rect(kind, viewport_width, viewport_height);
    let mut top = popup.origin.y + PADDING;
    let mut rects = Vec::new();
    for entry in menu_entries(kind) {
        let height = entry_height(*entry);
        rects.push((
            *entry,
            Rect::from_xywh(
                popup.origin.x + PADDING,
                top,
                popup.size.width - 2.0 * PADDING,
                height,
            ),
        ));
        top += height;
    }
    rects
}
