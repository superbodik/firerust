use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_inline_style::parse_inline_style;
use fr_css::value::align_items::AlignItems;
use fr_css::value::box_sizing::BoxSizing;
use fr_css::value::display::Display;
use fr_css::value::flex_direction::FlexDirection;
use fr_css::value::font_family::FontFamily;
use fr_css::value::font_style::FontStyle;
use fr_css::value::justify_content::JustifyContent;
use fr_css::value::length::Length;
use fr_css::value::line_height::LineHeight;
use fr_css::value::text_align::TextAlign;
use fr_css::value::white_space::WhiteSpace;

fn declarations_of(css: &str) -> Vec<Declaration> {
    parse_inline_style(css)
        .into_iter()
        .map(|entry| entry.declaration)
        .collect()
}

#[test]
fn sizes_are_read() {
    assert_eq!(
        declarations_of("width: 50%; height: 20px; min-width: 10em; max-width: 600px"),
        vec![
            Declaration::Width(Length::Percent(50.0)),
            Declaration::Height(Length::Px(20.0)),
            Declaration::MinWidth(Length::Em(10.0)),
            Declaration::MaxWidth(Length::Px(600.0)),
        ]
    );
}

#[test]
fn max_width_none_means_no_limit() {
    assert_eq!(
        declarations_of("max-width: none"),
        vec![Declaration::MaxWidth(Length::Auto)]
    );
}

#[test]
fn box_sizing_is_read() {
    assert_eq!(
        declarations_of("box-sizing: border-box"),
        vec![Declaration::BoxSizing(BoxSizing::BorderBox)]
    );
}

#[test]
fn text_properties_are_read() {
    assert_eq!(
        declarations_of("text-align: center; font-style: italic; white-space: pre-wrap"),
        vec![
            Declaration::TextAlign(TextAlign::Center),
            Declaration::FontStyle(FontStyle::Italic),
            Declaration::WhiteSpace(WhiteSpace::Pre),
        ]
    );
}

#[test]
fn the_first_known_font_family_wins() {
    assert_eq!(
        declarations_of("font-family: 'Fira Code', Consolas, monospace"),
        vec![Declaration::FontFamily(FontFamily::Monospace)]
    );
    assert_eq!(
        declarations_of("font-family: Georgia, serif"),
        vec![Declaration::FontFamily(FontFamily::Serif)]
    );
    assert!(declarations_of("font-family: 'Unknown Face'").is_empty());
}

#[test]
fn line_height_forms_are_read() {
    assert_eq!(
        declarations_of("line-height: 1.5"),
        vec![Declaration::LineHeight(LineHeight::Number(1.5))]
    );
    assert_eq!(
        declarations_of("line-height: 24px"),
        vec![Declaration::LineHeight(LineHeight::Px(24.0))]
    );
    assert_eq!(
        declarations_of("line-height: normal"),
        vec![Declaration::LineHeight(LineHeight::Normal)]
    );
}

#[test]
fn flex_container_properties_are_read() {
    assert_eq!(
        declarations_of(
            "display: flex; flex-direction: column; flex-wrap: wrap; justify-content: space-between; align-items: center"
        ),
        vec![
            Declaration::Display(Display::Flex),
            Declaration::FlexDirection(FlexDirection::Column),
            Declaration::FlexWrap(true),
            Declaration::JustifyContent(JustifyContent::SpaceBetween),
            Declaration::AlignItems(AlignItems::Center),
        ]
    );
}

#[test]
fn gap_with_two_values_sets_row_and_column() {
    assert_eq!(
        declarations_of("gap: 4px 8px"),
        vec![
            Declaration::RowGap(Length::Px(4.0)),
            Declaration::ColumnGap(Length::Px(8.0)),
        ]
    );
    assert_eq!(
        declarations_of("gap: 6px"),
        vec![
            Declaration::RowGap(Length::Px(6.0)),
            Declaration::ColumnGap(Length::Px(6.0)),
        ]
    );
}

#[test]
fn the_flex_shorthand_expands() {
    assert_eq!(
        declarations_of("flex: 1"),
        vec![
            Declaration::FlexGrow(1.0),
            Declaration::FlexShrink(1.0),
            Declaration::FlexBasis(Length::Px(0.0)),
        ]
    );
    assert_eq!(
        declarations_of("flex: 0 0 200px"),
        vec![
            Declaration::FlexGrow(0.0),
            Declaration::FlexShrink(0.0),
            Declaration::FlexBasis(Length::Px(200.0)),
        ]
    );
    assert_eq!(
        declarations_of("flex: none"),
        vec![
            Declaration::FlexGrow(0.0),
            Declaration::FlexShrink(0.0),
            Declaration::FlexBasis(Length::Auto),
        ]
    );
    assert_eq!(
        declarations_of("flex: auto"),
        vec![
            Declaration::FlexGrow(1.0),
            Declaration::FlexShrink(1.0),
            Declaration::FlexBasis(Length::Auto),
        ]
    );
}

#[test]
fn table_and_flex_display_values_are_read() {
    assert_eq!(
        declarations_of("display: table"),
        vec![Declaration::Display(Display::Table)]
    );
    assert_eq!(
        declarations_of("display: table-row"),
        vec![Declaration::Display(Display::TableRow)]
    );
    assert_eq!(
        declarations_of("display: table-cell"),
        vec![Declaration::Display(Display::TableCell)]
    );
    assert_eq!(
        declarations_of("display: inline-flex"),
        vec![Declaration::Display(Display::Flex)]
    );
}
