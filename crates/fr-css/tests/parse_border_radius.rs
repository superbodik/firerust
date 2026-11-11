use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_inline_style::parse_inline_style;
use fr_css::value::corner::Corner;
use fr_css::value::length::Length;

fn declarations_of(css: &str) -> Vec<Declaration> {
    parse_inline_style(css)
        .into_iter()
        .map(|entry| entry.declaration)
        .collect()
}

fn radius(corner: Corner, value: Length) -> Declaration {
    Declaration::BorderRadius(corner, value)
}

#[test]
fn one_value_rounds_every_corner() {
    assert_eq!(
        declarations_of("border-radius: 6px"),
        vec![
            radius(Corner::TopLeft, Length::Px(6.0)),
            radius(Corner::TopRight, Length::Px(6.0)),
            radius(Corner::BottomRight, Length::Px(6.0)),
            radius(Corner::BottomLeft, Length::Px(6.0)),
        ]
    );
}

#[test]
fn two_values_pair_up_the_diagonals() {
    assert_eq!(
        declarations_of("border-radius: 2px 8px"),
        vec![
            radius(Corner::TopLeft, Length::Px(2.0)),
            radius(Corner::TopRight, Length::Px(8.0)),
            radius(Corner::BottomRight, Length::Px(2.0)),
            radius(Corner::BottomLeft, Length::Px(8.0)),
        ]
    );
}

#[test]
fn three_and_four_values_follow_the_clockwise_order() {
    assert_eq!(
        declarations_of("border-radius: 1px 2px 3px"),
        vec![
            radius(Corner::TopLeft, Length::Px(1.0)),
            radius(Corner::TopRight, Length::Px(2.0)),
            radius(Corner::BottomRight, Length::Px(3.0)),
            radius(Corner::BottomLeft, Length::Px(2.0)),
        ]
    );
    assert_eq!(
        declarations_of("border-radius: 1px 2px 3px 4px"),
        vec![
            radius(Corner::TopLeft, Length::Px(1.0)),
            radius(Corner::TopRight, Length::Px(2.0)),
            radius(Corner::BottomRight, Length::Px(3.0)),
            radius(Corner::BottomLeft, Length::Px(4.0)),
        ]
    );
}

#[test]
fn percentages_and_font_relative_units_are_kept() {
    assert_eq!(
        declarations_of("border-radius: 50%"),
        vec![
            radius(Corner::TopLeft, Length::Percent(50.0)),
            radius(Corner::TopRight, Length::Percent(50.0)),
            radius(Corner::BottomRight, Length::Percent(50.0)),
            radius(Corner::BottomLeft, Length::Percent(50.0)),
        ]
    );
    assert_eq!(
        declarations_of("border-top-left-radius: 0.5em"),
        vec![radius(Corner::TopLeft, Length::Em(0.5))]
    );
}

#[test]
fn the_elliptical_part_after_a_slash_is_ignored() {
    assert_eq!(
        declarations_of("border-radius: 4px / 10px"),
        vec![
            radius(Corner::TopLeft, Length::Px(4.0)),
            radius(Corner::TopRight, Length::Px(4.0)),
            radius(Corner::BottomRight, Length::Px(4.0)),
            radius(Corner::BottomLeft, Length::Px(4.0)),
        ]
    );
}

#[test]
fn a_longhand_takes_the_first_value_only() {
    assert_eq!(
        declarations_of("border-bottom-right-radius: 9px 3px"),
        vec![radius(Corner::BottomRight, Length::Px(9.0))]
    );
}

#[test]
fn a_negative_or_broken_radius_is_dropped() {
    assert!(declarations_of("border-radius: -4px").is_empty());
    assert!(declarations_of("border-radius: red").is_empty());
    assert!(declarations_of("border-radius: 1px 2px 3px 4px 5px").is_empty());
}
