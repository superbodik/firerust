use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::value::display::Display;

#[test]
fn rules_and_declarations_are_read() {
    let sheet = parse_stylesheet("p { color: red; font-weight: bold } div { display: none }");

    assert_eq!(sheet.rules.len(), 2);
    assert_eq!(sheet.rules[0].declarations.len(), 2);
    assert_eq!(
        sheet.rules[1].declarations[0].declaration,
        Declaration::Display(Display::None)
    );
}

#[test]
fn a_selector_list_gives_several_selectors() {
    let sheet = parse_stylesheet("h1, h2, .big { color: red }");

    assert_eq!(sheet.rules.len(), 1);
    assert_eq!(sheet.rules[0].selectors.len(), 3);
}

#[test]
fn comments_are_ignored() {
    let sheet = parse_stylesheet("/* top */ p { /* inside */ color: red; } /* end */");

    assert_eq!(sheet.rules.len(), 1);
    assert_eq!(sheet.rules[0].declarations.len(), 1);
}

#[test]
fn at_rules_with_blocks_are_skipped_entirely() {
    let sheet =
        parse_stylesheet("@media (min-width: 10px) { p { color: red } } div { color: blue }");

    assert_eq!(sheet.rules.len(), 1);
}

#[test]
fn at_rules_with_a_semicolon_are_skipped() {
    let sheet = parse_stylesheet("@import url(other.css); @charset \"utf-8\"; p { color: red }");

    assert_eq!(sheet.rules.len(), 1);
}

#[test]
fn an_unsupported_selector_drops_only_itself() {
    let sheet = parse_stylesheet("a:hover, p { color: red }");

    assert_eq!(sheet.rules.len(), 1);
    assert_eq!(sheet.rules[0].selectors.len(), 1);
}

#[test]
fn a_rule_with_only_unsupported_selectors_is_dropped() {
    let sheet = parse_stylesheet(
        "a:hover { color: red } input[type=text] { color: red } p { color: blue }",
    );

    assert_eq!(sheet.rules.len(), 1);
}

#[test]
fn a_garbage_rule_does_not_hide_the_next_one() {
    let sheet = parse_stylesheet("}}} ;; p { color: red }");

    assert_eq!(sheet.rules.len(), 1);
}

#[test]
fn an_empty_sheet_has_no_rules() {
    assert!(parse_stylesheet("").rules.is_empty());
    assert!(parse_stylesheet("   \n  ").rules.is_empty());
}

#[test]
fn an_unterminated_block_does_not_hang() {
    let sheet = parse_stylesheet("p { color: red");

    assert_eq!(sheet.rules.len(), 1);
}
