use crate::cascade::computed_style::ComputedStyle;
use crate::cascade::specified_style::SpecifiedStyle;
use crate::value::display::Display;
use crate::value::length::{Length, ROOT_FONT_SIZE};

const MIN_FONT_SIZE: f32 = 1.0;

pub fn compute_style(specified: &SpecifiedStyle, parent: &ComputedStyle) -> ComputedStyle {
    let font_size = resolve_font_size(specified.font_size, parent.font_size);
    let color = specified.color.unwrap_or(parent.color);
    ComputedStyle {
        display: specified.display.unwrap_or(Display::Inline),
        color,
        background_color: specified
            .background_color
            .unwrap_or(ComputedStyle::initial().background_color),
        font_size,
        font_weight: specified.font_weight.unwrap_or(parent.font_weight),
        margin: specified
            .margin
            .map(|value| value.unwrap_or(Length::Px(0.0)).to_font_relative(font_size)),
        padding: specified
            .padding
            .map(|value| value.unwrap_or(Length::Px(0.0)).to_font_relative(font_size)),
        border_width: specified.border_width.map(|value| {
            value.map_or(0.0, |length| {
                length.to_font_relative(font_size).px_or_zero()
            })
        }),
        border_color: specified.border_color.unwrap_or(color),
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
