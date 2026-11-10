use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_inline_style::parse_inline_style;
use fr_css::value::display::Display;
use fr_css::value::length::Length;
use fr_css::value::side::Side;
use fr_geom::color::rgba::Rgba;

fn declarations_of(css: &str) -> Vec<Declaration> {
    parse_inline_style(css)
        .into_iter()
        .map(|entry| entry.declaration)
        .collect()
}

#[test]
fn a_margin_with_one_value_covers_every_side() {
    assert_eq!(
        declarations_of("margin: 4px"),
        vec![
            Declaration::Margin(Side::Top, Length::Px(4.0)),
            Declaration::Margin(Side::Right, Length::Px(4.0)),
            Declaration::Margin(Side::Bottom, Length::Px(4.0)),
            Declaration::Margin(Side::Left, Length::Px(4.0)),
        ]
    );
}

#[test]
fn a_margin_with_two_values_pairs_vertical_and_horizontal() {
    assert_eq!(
        declarations_of("margin: 1em 2px"),
        vec![
            Declaration::Margin(Side::Top, Length::Em(1.0)),
            Declaration::Margin(Side::Right, Length::Px(2.0)),
            Declaration::Margin(Side::Bottom, Length::Em(1.0)),
            Declaration::Margin(Side::Left, Length::Px(2.0)),
        ]
    );
}

#[test]
fn a_margin_with_four_values_follows_clockwise_order() {
    assert_eq!(
        declarations_of("margin: 1px 2px 3px 4px"),
        vec![
            Declaration::Margin(Side::Top, Length::Px(1.0)),
            Declaration::Margin(Side::Right, Length::Px(2.0)),
            Declaration::Margin(Side::Bottom, Length::Px(3.0)),
            Declaration::Margin(Side::Left, Length::Px(4.0)),
        ]
    );
}

#[test]
fn a_margin_with_auto_keeps_auto() {
    assert_eq!(
        declarations_of("margin-left: auto"),
        vec![Declaration::Margin(Side::Left, Length::Auto)]
    );
}

#[test]
fn a_single_side_padding_is_read() {
    assert_eq!(
        declarations_of("padding-left: 40px"),
        vec![Declaration::Padding(Side::Left, Length::Px(40.0))]
    );
}

#[test]
fn a_border_shorthand_sets_width_and_color() {
    assert_eq!(
        declarations_of("border-bottom: 2px solid #ccc"),
        vec![
            Declaration::BorderWidth(Side::Bottom, Length::Px(2.0)),
            Declaration::BorderColor(Rgba::opaque(204, 204, 204)),
        ]
    );
}

#[test]
fn a_border_without_a_style_has_no_width() {
    assert_eq!(
        declarations_of("border: 3px"),
        vec![
            Declaration::BorderWidth(Side::Top, Length::Px(0.0)),
            Declaration::BorderWidth(Side::Right, Length::Px(0.0)),
            Declaration::BorderWidth(Side::Bottom, Length::Px(0.0)),
            Declaration::BorderWidth(Side::Left, Length::Px(0.0)),
        ]
    );
}

#[test]
fn a_border_none_has_no_width() {
    assert_eq!(
        declarations_of("border-top: none"),
        vec![Declaration::BorderWidth(Side::Top, Length::Px(0.0))]
    );
}

#[test]
fn font_weight_keywords_and_numbers_are_read() {
    assert_eq!(
        declarations_of("font-weight: bold"),
        vec![Declaration::FontWeight(700)]
    );
    assert_eq!(
        declarations_of("font-weight: normal"),
        vec![Declaration::FontWeight(400)]
    );
    assert_eq!(
        declarations_of("font-weight: 600"),
        vec![Declaration::FontWeight(600)]
    );
}

#[test]
fn display_keywords_are_read() {
    assert_eq!(
        declarations_of("display: none"),
        vec![Declaration::Display(Display::None)]
    );
    assert_eq!(
        declarations_of("display: block"),
        vec![Declaration::Display(Display::Block)]
    );
    assert_eq!(
        declarations_of("display: inline-block"),
        vec![Declaration::Display(Display::Inline)]
    );
}

#[test]
fn a_background_shorthand_takes_the_color() {
    assert_eq!(
        declarations_of("background: url(x.png) no-repeat #ffcc00"),
        vec![Declaration::BackgroundColor(Rgba::opaque(255, 204, 0))]
    );
}

#[test]
fn units_are_read() {
    assert_eq!(
        declarations_of("font-size: 12pt"),
        vec![Declaration::FontSize(Length::Px(16.0))]
    );
    assert_eq!(
        declarations_of("font-size: 1.5rem"),
        vec![Declaration::FontSize(Length::Rem(1.5))]
    );
    assert_eq!(
        declarations_of("font-size: 120%"),
        vec![Declaration::FontSize(Length::Percent(120.0))]
    );
}

#[test]
fn important_is_recorded() {
    let parsed = parse_inline_style("color: red !important; margin-top: 1px");

    assert!(parsed[0].is_important);
    assert!(!parsed[1].is_important);
}

#[test]
fn a_bad_declaration_does_not_stop_the_next_one() {
    assert_eq!(
        declarations_of("color: banana; font-weight: bold; unknown: 1; display: none"),
        vec![
            Declaration::FontWeight(700),
            Declaration::Display(Display::None),
        ]
    );
}

#[test]
fn a_value_with_extra_tokens_is_dropped() {
    assert_eq!(
        declarations_of("font-size: 12px 14px; display: block"),
        vec![Declaration::Display(Display::Block)]
    );
}

#[test]
fn property_names_are_case_insensitive() {
    assert_eq!(
        declarations_of("DISPLAY: NONE"),
        vec![Declaration::Display(Display::None)]
    );
}

#[test]
fn an_empty_block_has_no_declarations() {
    assert!(declarations_of("").is_empty());
    assert!(declarations_of("  ;;  ").is_empty());
}
