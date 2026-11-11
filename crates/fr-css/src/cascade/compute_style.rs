use crate::cascade::computed_style::ComputedStyle;
use crate::cascade::specified_style::SpecifiedStyle;
use crate::value::display::Display;
use crate::value::flex_style::FlexStyle;
use crate::value::length::{Length, ROOT_FONT_SIZE};
use crate::value::size_style::SizeStyle;

const MIN_FONT_SIZE: f32 = 1.0;

pub fn compute_style(specified: &SpecifiedStyle, parent: &ComputedStyle) -> ComputedStyle {
    let font_size = resolve_font_size(specified.font_size, parent.font_size);
    let color = specified.color.unwrap_or(parent.color);
    let relative = |value: Option<Length>, default: Length| {
        value.unwrap_or(default).to_font_relative(font_size)
    };
    let gap = |value: Option<Length>| relative(value, Length::Px(0.0)).px_or_zero();

    ComputedStyle {
        display: specified.display.unwrap_or(Display::Inline),
        color,
        background_color: specified
            .background_color
            .unwrap_or(ComputedStyle::initial().background_color),
        font_size,
        font_weight: specified.font_weight.unwrap_or(parent.font_weight),
        font_style: specified.font_style.unwrap_or(parent.font_style),
        font_family: specified.font_family.unwrap_or(parent.font_family),
        line_height: specified.line_height.unwrap_or(parent.line_height),
        text_align: specified.text_align.unwrap_or(parent.text_align),
        white_space: specified.white_space.unwrap_or(parent.white_space),
        box_sizing: specified.box_sizing.unwrap_or_default(),
        size: SizeStyle {
            width: relative(specified.width, Length::Auto),
            height: relative(specified.height, Length::Auto),
            min_width: relative(specified.min_width, Length::Auto),
            max_width: relative(specified.max_width, Length::Auto),
        },
        margin: specified
            .margin
            .map(|value| relative(value, Length::Px(0.0))),
        padding: specified
            .padding
            .map(|value| relative(value, Length::Px(0.0))),
        border_width: specified
            .border_width
            .map(|value| relative(value, Length::Px(0.0)).px_or_zero()),
        border_color: specified.border_color.unwrap_or(color),
        accent_color: specified.accent_color.or(parent.accent_color),
        border_radius: specified
            .border_radius
            .map(|value| relative(value, Length::Px(0.0))),
        flex: FlexStyle {
            direction: specified.flex_direction.unwrap_or_default(),
            is_wrapping: specified.flex_wrap.unwrap_or(false),
            justify_content: specified.justify_content.unwrap_or_default(),
            align_items: specified.align_items.unwrap_or_default(),
            row_gap: gap(specified.row_gap),
            column_gap: gap(specified.column_gap),
            grow: specified.flex_grow.unwrap_or(0.0),
            shrink: specified.flex_shrink.unwrap_or(1.0),
            basis: relative(specified.flex_basis, Length::Auto),
        },
    }
}

fn resolve_font_size(specified: Option<Length>, parent: f32) -> f32 {
    let size = match specified {
        Some(Length::Px(value)) => value,
        Some(Length::Em(value)) => value * parent,
        Some(Length::Rem(value)) => value * ROOT_FONT_SIZE,
        Some(Length::Percent(value)) => parent * value / 100.0,
        Some(Length::Auto) | None => parent,
    };
    size.max(MIN_FONT_SIZE)
}
