use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::measure_runs::measure_runs;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;

use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::boxes::layout_run::LayoutRun;
use crate::boxes::text_box::TextBox;
use crate::control::control_kind::ControlKind;

const TEXT_INSET: f32 = 4.0;
const MARK_INSET: f32 = 1.0;
const MARK_FONT_SIZE: f32 = 11.0;

pub fn control_text_box(
    engine: &mut TextEngine,
    node: NodeId,
    kind: ControlKind,
    style: &ComputedStyle,
    text: &str,
    color: Rgba,
    frame: Rect,
) -> Option<LayoutBox> {
    let is_mark = matches!(kind, ControlKind::Checkbox | ControlKind::Radio);
    let inset = if is_mark { MARK_INSET } else { TEXT_INSET };
    let inner_width = (frame.size.width - 2.0 * inset).max(1.0);
    let is_single_line = !matches!(kind, ControlKind::TextArea);
    let mut shown = text.to_owned();
    if is_single_line && !matches!(kind, ControlKind::Button | ControlKind::Select) && !is_mark {
        shown = trim_to_fit(engine, &shown, style, color, inner_width);
    }
    if shown.is_empty() {
        return None;
    }
    let mut text_style = *style;
    text_style.color = color;
    if is_mark {
        text_style.font_size = MARK_FONT_SIZE;
        text_style.line_height = fr_css::value::line_height::LineHeight::Normal;
    }
    let run = LayoutRun::from_style(shown, &text_style).with_owner(Some(node));
    let alignment = match kind {
        ControlKind::Button | ControlKind::Checkbox | ControlKind::Radio => TextAlignment::Center,
        _ => TextAlignment::Left,
    };
    let height = shape_runs(engine, &[run.as_text_run()], inner_width, alignment).height();
    let top = if is_single_line {
        frame.origin.y + ((frame.size.height - height) / 2.0).max(0.0)
    } else {
        frame.origin.y + TEXT_INSET
    };
    let rect = Rect::from_xywh(frame.origin.x + inset, top, inner_width, height);
    Some(LayoutBox::plain(
        None,
        BoxKind::Text(TextBox {
            runs: vec![run],
            alignment,
        }),
        rect,
    ))
}

fn trim_to_fit(
    engine: &mut TextEngine,
    text: &str,
    style: &ComputedStyle,
    color: Rgba,
    width: f32,
) -> String {
    let mut text_style = *style;
    text_style.color = color;
    let mut characters: Vec<char> = text.chars().collect();
    loop {
        let candidate: String = characters.iter().collect();
        let run = LayoutRun::from_style(candidate, &text_style);
        let widths = measure_runs(engine, &[run.as_text_run()]);
        if widths.max <= width || characters.is_empty() {
            return characters.iter().collect();
        }
        characters.remove(0);
    }
}
