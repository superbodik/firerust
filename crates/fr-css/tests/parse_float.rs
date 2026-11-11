use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_inline_style::parse_inline_style;
use fr_css::value::clear_side::ClearSide;
use fr_css::value::float_side::FloatSide;

fn declarations_of(css: &str) -> Vec<Declaration> {
    parse_inline_style(css)
        .into_iter()
        .map(|entry| entry.declaration)
        .collect()
}

#[test]
fn float_values_are_read() {
    assert_eq!(
        declarations_of("float: left"),
        vec![Declaration::Float(FloatSide::Left)]
    );
    assert_eq!(
        declarations_of("float: right"),
        vec![Declaration::Float(FloatSide::Right)]
    );
    assert_eq!(
        declarations_of("float: none"),
        vec![Declaration::Float(FloatSide::None)]
    );
    assert_eq!(
        declarations_of("float: inline-start"),
        vec![Declaration::Float(FloatSide::Left)]
    );
    assert_eq!(
        declarations_of("float: inline-end"),
        vec![Declaration::Float(FloatSide::Right)]
    );
}

#[test]
fn clear_values_are_read() {
    assert_eq!(
        declarations_of("clear: both"),
        vec![Declaration::Clear(ClearSide::Both)]
    );
    assert_eq!(
        declarations_of("clear: left"),
        vec![Declaration::Clear(ClearSide::Left)]
    );
    assert_eq!(
        declarations_of("clear: right"),
        vec![Declaration::Clear(ClearSide::Right)]
    );
    assert_eq!(
        declarations_of("clear: none"),
        vec![Declaration::Clear(ClearSide::None)]
    );
}

#[test]
fn unknown_float_and_clear_values_are_dropped() {
    assert!(declarations_of("float: sideways").is_empty());
    assert!(declarations_of("clear: up").is_empty());
}

#[test]
fn clear_sides_know_which_floats_they_clear() {
    assert!(ClearSide::Both.clears_left() && ClearSide::Both.clears_right());
    assert!(ClearSide::Left.clears_left() && !ClearSide::Left.clears_right());
    assert!(!ClearSide::Right.clears_left() && ClearSide::Right.clears_right());
    assert!(!ClearSide::None.clears_left() && !ClearSide::None.clears_right());
}
