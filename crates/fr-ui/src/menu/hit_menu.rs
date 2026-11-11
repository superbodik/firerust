use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;

use crate::menu::entry_rects::entry_rects;
use crate::menu::menu_entry::MenuEntry;
use crate::menu::menu_hit::MenuHit;
use crate::menu::menu_kind::MenuKind;
use crate::menu::menu_metrics::menu_rect;
use crate::menu::menu_target::MenuTarget;
use crate::menu::zoom_in_rect::zoom_in_rect;
use crate::menu::zoom_out_rect::zoom_out_rect;
use crate::menu::zoom_value_rect::zoom_value_rect;

pub fn hit_menu(
    kind: MenuKind,
    point: Point,
    viewport_width: f32,
    viewport_height: f32,
) -> MenuHit {
    if !menu_rect(kind, viewport_width, viewport_height).contains(point) {
        return MenuHit::Outside;
    }
    for (entry, rect) in entry_rects(kind, viewport_width, viewport_height) {
        if !rect.contains(point) {
            continue;
        }
        return match entry {
            MenuEntry::Item(target) => MenuHit::Target(target),
            MenuEntry::Separator => MenuHit::Inert,
            MenuEntry::Zoom => zoom_hit(rect, point),
        };
    }
    MenuHit::Inert
}

fn zoom_hit(row: Rect, point: Point) -> MenuHit {
    if zoom_out_rect(row).contains(point) {
        MenuHit::Target(MenuTarget::ZoomOut)
    } else if zoom_in_rect(row).contains(point) {
        MenuHit::Target(MenuTarget::ZoomIn)
    } else if zoom_value_rect(row).contains(point) {
        MenuHit::Target(MenuTarget::ZoomReset)
    } else {
        MenuHit::Inert
    }
}
