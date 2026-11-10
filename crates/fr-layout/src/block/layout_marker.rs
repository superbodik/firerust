use fr_css::cascade::computed_style::ComputedStyle;
use fr_geom::geometry::rect::Rect;
use fr_text::render::text_alignment::TextAlignment;

use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::boxes::layout_run::LayoutRun;
use crate::boxes::text_box::TextBox;

const MARKER_GAP_FACTOR: f32 = 0.4;

pub fn layout_marker(style: &ComputedStyle, content_x: f32, content_y: f32) -> LayoutBox {
    let run = LayoutRun::from_style("\u{2022}".to_owned(), style);
    let width = style.font_size;
    let gap = style.font_size * MARKER_GAP_FACTOR;
    let rect = Rect::from_xywh(content_x - width - gap, content_y, width, run.line_height);
    LayoutBox::plain(
        None,
        BoxKind::Marker(TextBox {
            runs: vec![run],
            alignment: TextAlignment::Right,
        }),
        rect,
    )
}
