use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_inline_style::parse_inline_style;
use fr_css::value::text_decoration::TextDecoration;

fn declarations_of(css: &str) -> Vec<Declaration> {
    parse_inline_style(css)
        .into_iter()
        .map(|entry| entry.declaration)
        .collect()
}

const UNDERLINE: TextDecoration = TextDecoration {
    underline: true,
    line_through: false,
};

#[test]
fn underline_and_line_through_are_read() {
    assert_eq!(
        declarations_of("text-decoration: underline"),
        vec![Declaration::TextDecoration(UNDERLINE)]
    );
    assert_eq!(
        declarations_of("text-decoration: line-through"),
        vec![Declaration::TextDecoration(TextDecoration {
            underline: false,
            line_through: true
        })]
    );
}

#[test]
fn both_lines_can_be_asked_for_at_once() {
    assert_eq!(
        declarations_of("text-decoration: underline line-through"),
        vec![Declaration::TextDecoration(TextDecoration {
            underline: true,
            line_through: true
        })]
    );
}

#[test]
fn none_clears_the_decoration() {
    assert_eq!(
        declarations_of("text-decoration: none"),
        vec![Declaration::TextDecoration(TextDecoration::NONE)]
    );
}

#[test]
fn style_and_colour_parts_of_the_shorthand_are_ignored() {
    assert_eq!(
        declarations_of("text-decoration: underline dotted #ff0000"),
        vec![Declaration::TextDecoration(UNDERLINE)]
    );
    assert_eq!(
        declarations_of("text-decoration-line: underline"),
        vec![Declaration::TextDecoration(UNDERLINE)]
    );
}

#[test]
fn a_value_without_a_line_keyword_is_dropped() {
    assert!(declarations_of("text-decoration: dotted").is_empty());
}
