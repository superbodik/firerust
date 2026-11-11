use fr_css::media::media_matches::media_matches;
use fr_css::parse::parse_stylesheet::parse_stylesheet;

fn rule_count(css: &str) -> usize {
    parse_stylesheet(css).rules.len()
}

#[test]
fn width_conditions_compare_with_the_nominal_desktop_viewport() {
    assert!(media_matches("(min-width: 768px)"));
    assert!(media_matches("(min-width: 1024px)"));
    assert!(!media_matches("(min-width: 1400px)"));
    assert!(media_matches("(max-width: 1200px)"));
    assert!(!media_matches("(max-width: 600px)"));
    assert!(media_matches("(min-width: 40em) and (max-width: 80em)"));
    assert!(!media_matches("(min-width: 40em) and (max-width: 60em)"));
}

#[test]
fn media_types_are_screen_friendly() {
    assert!(media_matches("screen"));
    assert!(media_matches("all"));
    assert!(media_matches("only screen and (min-width: 600px)"));
    assert!(!media_matches("print"));
    assert!(!media_matches("not screen"));
    assert!(media_matches("not print"));
}

#[test]
fn a_list_matches_when_any_query_does() {
    assert!(media_matches("print, (min-width: 600px)"));
    assert!(!media_matches("print, (max-width: 300px)"));
}

#[test]
fn an_empty_condition_applies() {
    assert!(media_matches(""));
}

#[test]
fn user_preference_features_describe_a_plain_light_desktop() {
    assert!(media_matches("(prefers-color-scheme: light)"));
    assert!(!media_matches("(prefers-color-scheme: dark)"));
    assert!(media_matches("(prefers-reduced-motion: no-preference)"));
    assert!(!media_matches("(prefers-reduced-motion: reduce)"));
    assert!(media_matches("(hover: hover) and (pointer: fine)"));
    assert!(!media_matches("(pointer: coarse)"));
    assert!(media_matches("(orientation: landscape)"));
}

#[test]
fn resolution_is_one_device_pixel_per_css_pixel() {
    assert!(media_matches("(min-resolution: 1dppx)"));
    assert!(!media_matches("(min-resolution: 2dppx)"));
    assert!(!media_matches("(-webkit-min-device-pixel-ratio: 2)"));
    assert!(media_matches("(min-resolution: 96dpi)"));
}

#[test]
fn range_syntax_is_understood() {
    assert!(media_matches("(width >= 600px)"));
    assert!(!media_matches("(width < 600px)"));
    assert!(media_matches("(400px <= width <= 1200px)"));
    assert!(!media_matches("(400px <= width <= 800px)"));
}

#[test]
fn unknown_features_do_not_match() {
    assert!(!media_matches("(made-up-feature: yes)"));
}

#[test]
fn rules_inside_a_matching_media_block_are_kept_in_order() {
    let sheet = parse_stylesheet(
        "p { color: red } @media (min-width: 600px) { p { color: blue } h1 { color: green } } b { color: red }",
    );

    assert_eq!(sheet.rules.len(), 4);
}

#[test]
fn rules_inside_a_failing_media_block_are_dropped() {
    assert_eq!(
        rule_count(
            "p { color: red } @media (max-width: 300px) { p { color: blue } } b { color: red }"
        ),
        2
    );
    assert_eq!(
        rule_count("@media print { p { color: blue } } a { color: red }"),
        1
    );
}

#[test]
fn nested_media_blocks_work() {
    assert_eq!(
        rule_count("@media screen { @media (min-width: 600px) { p { color: red } } }"),
        1
    );
}

#[test]
fn supports_and_layer_blocks_are_included() {
    assert_eq!(
        rule_count("@supports (display: grid) { p { color: red } }"),
        1
    );
    assert_eq!(
        rule_count("@supports not (display: grid) { p { color: red } }"),
        0
    );
    assert_eq!(rule_count("@layer base { p { color: red } }"), 1);
    assert_eq!(rule_count("@layer base, theme; p { color: red }"), 1);
}

#[test]
fn other_at_rules_are_skipped_without_hiding_what_follows() {
    assert_eq!(
        rule_count(
            "@font-face { font-family: x; src: url(a.woff) } @keyframes spin { from { color: red } to { color: blue } } @import url(x.css); p { color: red }"
        ),
        1
    );
}
