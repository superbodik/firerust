use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::display::Display;
use fr_css::value::edges::Edges;
use fr_css::value::flex_style::FlexStyle;
use fr_css::value::length::Length;
use fr_css::value::size_style::SizeStyle;
use fr_geom::color::rgba::Rgba;

pub fn anonymous_style(container: &ComputedStyle) -> ComputedStyle {
    ComputedStyle {
        display: Display::Block,
        background_color: Rgba::new(0, 0, 0, 0),
        size: SizeStyle::default(),
        margin: Edges::uniform(Length::Px(0.0)),
        padding: Edges::uniform(Length::Px(0.0)),
        border_width: Edges::uniform(0.0),
        flex: FlexStyle::default(),
        ..*container
    }
}
