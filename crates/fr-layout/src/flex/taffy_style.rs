use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::align_items::AlignItems as CssAlignItems;
use fr_css::value::box_sizing::BoxSizing as CssBoxSizing;
use fr_css::value::flex_direction::FlexDirection as CssFlexDirection;
use fr_css::value::justify_content::JustifyContent as CssJustifyContent;
use fr_css::value::length::Length;
use taffy::prelude::{
    AlignItems, BoxSizing, Display, FlexDirection, FlexWrap, JustifyContent, Rect, Size, Style,
    auto, length,
};

use crate::flex::taffy_convert::{dimension, length_percentage, length_percentage_auto};

pub fn taffy_item_style(style: &ComputedStyle) -> Style {
    Style {
        display: Display::Flex,
        box_sizing: match style.box_sizing {
            CssBoxSizing::ContentBox => BoxSizing::ContentBox,
            CssBoxSizing::BorderBox => BoxSizing::BorderBox,
        },
        size: Size {
            width: dimension(style.size.width),
            height: dimension(style.size.height),
        },
        min_size: Size {
            width: length_percentage_auto(style.size.min_width),
            height: auto(),
        },
        max_size: Size {
            width: length_percentage_auto(style.size.max_width),
            height: auto(),
        },
        margin: Rect {
            left: length_percentage_auto(style.margin.left),
            right: length_percentage_auto(style.margin.right),
            top: length_percentage_auto(style.margin.top),
            bottom: length_percentage_auto(style.margin.bottom),
        },
        padding: Rect {
            left: length_percentage(style.padding.left),
            right: length_percentage(style.padding.right),
            top: length_percentage(style.padding.top),
            bottom: length_percentage(style.padding.bottom),
        },
        border: Rect {
            left: length(style.border_width.left),
            right: length(style.border_width.right),
            top: length(style.border_width.top),
            bottom: length(style.border_width.bottom),
        },
        flex_grow: style.flex.grow,
        flex_shrink: style.flex.shrink,
        flex_basis: dimension(style.flex.basis),
        ..Style::default()
    }
}

pub fn taffy_container_style(
    style: &ComputedStyle,
    content_width: f32,
    explicit_height: Option<f32>,
) -> Style {
    Style {
        display: Display::Flex,
        size: Size {
            width: length(content_width),
            height: explicit_height.map_or_else(|| dimension(Length::Auto), length),
        },
        flex_direction: match style.flex.direction {
            CssFlexDirection::Row => FlexDirection::Row,
            CssFlexDirection::RowReverse => FlexDirection::RowReverse,
            CssFlexDirection::Column => FlexDirection::Column,
            CssFlexDirection::ColumnReverse => FlexDirection::ColumnReverse,
        },
        flex_wrap: if style.flex.is_wrapping {
            FlexWrap::Wrap
        } else {
            FlexWrap::NoWrap
        },
        justify_content: match style.flex.justify_content {
            CssJustifyContent::FlexStart => JustifyContent::FLEX_START,
            CssJustifyContent::FlexEnd => JustifyContent::FLEX_END,
            CssJustifyContent::Center => JustifyContent::CENTER,
            CssJustifyContent::SpaceBetween => JustifyContent::SPACE_BETWEEN,
            CssJustifyContent::SpaceAround => JustifyContent::SPACE_AROUND,
            CssJustifyContent::SpaceEvenly => JustifyContent::SPACE_EVENLY,
        },
        align_items: match style.flex.align_items {
            CssAlignItems::Stretch => AlignItems::STRETCH,
            CssAlignItems::FlexStart => AlignItems::FLEX_START,
            CssAlignItems::FlexEnd => AlignItems::FLEX_END,
            CssAlignItems::Center => AlignItems::CENTER,
        },
        gap: Size {
            width: length(style.flex.column_gap),
            height: length(style.flex.row_gap),
        },
        ..Style::default()
    }
}
