mod support;

use support::{Painted, paint};

fn ink(page: &Painted) -> usize {
    let mut count = 0;
    for y in 0..support::HEIGHT {
        for x in 0..support::WIDTH {
            if page.rgb(x, y) != (255, 255, 255) {
                count += 1;
            }
        }
    }
    count
}

const LINK: &str = "<p><a href='#'>wwwwwwwwww</a></p>";

#[test]
fn a_link_is_underlined_by_default() {
    let underlined = paint(LINK, "", 0.0);
    let plain = paint(LINK, "a { text-decoration: none }", 0.0);

    assert!(ink(&underlined) > ink(&plain) + 40);
}

#[test]
fn text_decoration_can_underline_any_text() {
    let underlined = paint("<p>wwwwwwwwww</p>", "p { text-decoration: underline }", 0.0);
    let plain = paint("<p>wwwwwwwwww</p>", "", 0.0);

    assert!(ink(&underlined) > ink(&plain) + 40);
}

#[test]
fn line_through_adds_a_line_over_the_text() {
    let struck = paint(
        "<p>............</p>",
        "p { text-decoration: line-through }",
        0.0,
    );
    let plain = paint("<p>............</p>", "", 0.0);

    assert!(ink(&struck) > ink(&plain) + 20);
}

#[test]
fn the_decoration_is_inherited_by_children() {
    let nested = paint(
        "<p><span>wwwwwwwwww</span></p>",
        "p { text-decoration: underline }",
        0.0,
    );
    let plain = paint("<p><span>wwwwwwwwww</span></p>", "", 0.0);

    assert!(ink(&nested) > ink(&plain) + 40);
}
