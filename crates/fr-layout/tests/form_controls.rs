mod support;

use fr_geom::color::rgba::Rgba;
use fr_layout::boxes::box_kind::BoxKind;
use fr_layout::boxes::layout_box::LayoutBox;
use fr_layout::control::control_kind::ControlKind;
use fr_layout::control::control_kind_of::control_kind_of;
use support::{layout, layout_focused};

fn text_of(boxed: &LayoutBox) -> String {
    boxed
        .children
        .iter()
        .filter_map(|child| match &child.kind {
            BoxKind::Text(text) => Some(text.plain_text()),
            _ => None,
        })
        .collect()
}

#[test]
fn a_text_input_is_a_white_bordered_box_showing_its_value() {
    let page = layout("<input id=i type=text value='hello'>", "", 800.0);

    let field = page.boxed("i");
    assert!(field.border_box.size.width > 100.0);
    assert!(field.border_box.size.height > 15.0);
    assert_eq!(field.background, Rgba::opaque(255, 255, 255));
    assert_eq!(field.border_width.left, 1.0);
    assert_eq!(text_of(field), "hello");
}

#[test]
fn an_input_without_a_type_is_a_text_input() {
    let page = layout("<input id=i value='x'>", "", 800.0);

    assert_eq!(text_of(page.boxed("i")), "x");
}

#[test]
fn an_empty_input_with_a_placeholder_shows_it_in_grey() {
    let page = layout("<input id=i placeholder='Your name'>", "", 800.0);

    let field = page.boxed("i");
    assert_eq!(text_of(field), "Your name");
    let BoxKind::Text(text) = &field.children[0].kind else {
        panic!("text child");
    };
    assert_eq!(text.runs[0].color, Rgba::opaque(140, 140, 140));
}

#[test]
fn a_password_is_masked() {
    let page = layout("<input id=i type=password value='secret'>", "", 800.0);

    assert_eq!(text_of(page.boxed("i")), "••••••");
}

#[test]
fn a_hidden_input_has_no_box() {
    let page = layout(
        "<input id=i type=hidden value='x'><p id=p>after</p>",
        "",
        800.0,
    );

    assert!(page.find("i").is_none());
    assert!(page.find("p").is_some());
}

#[test]
fn a_checked_checkbox_shows_a_mark_and_an_unchecked_one_does_not() {
    let page = layout(
        "<input id=a type=checkbox checked><input id=b type=checkbox>",
        "",
        800.0,
    );

    assert_eq!(text_of(page.boxed("a")), "✓");
    assert_eq!(text_of(page.boxed("b")), "");
    assert_eq!(page.boxed("a").border_box.size.width, 14.0);
}

#[test]
fn a_radio_button_shows_a_dot_when_checked() {
    let page = layout("<input id=r type=radio checked>", "", 800.0);

    assert_eq!(text_of(page.boxed("r")), "●");
}

#[test]
fn a_submit_button_has_a_default_label_and_a_grey_background() {
    let page = layout("<input id=b type=submit>", "", 800.0);

    let button = page.boxed("b");
    assert_eq!(text_of(button), "Submit");
    assert_eq!(button.background, Rgba::opaque(233, 233, 237));
    assert!(button.border_box.size.width >= 48.0);
}

#[test]
fn a_button_input_uses_its_value_as_the_label() {
    let page = layout("<input id=b type=button value='Press me'>", "", 800.0);

    assert_eq!(text_of(page.boxed("b")), "Press me");
}

#[test]
fn a_button_element_uses_its_text() {
    let page = layout("<button id=b>  Save   it </button>", "", 800.0);

    assert_eq!(text_of(page.boxed("b")), "Save it");
}

#[test]
fn a_block_button_keeps_its_css_box() {
    let page = layout(
        "<button id=b>Go</button>",
        "button { display: block; background-color: #ff0000; padding: 5px }",
        800.0,
    );

    assert_eq!(page.boxed("b").background, Rgba::opaque(255, 0, 0));
}

#[test]
fn a_select_shows_the_first_option_by_default() {
    let page = layout(
        "<select id=s><option>One</option><option>Two</option></select>",
        "",
        800.0,
    );

    assert_eq!(text_of(page.boxed("s")), "One ▾");
}

#[test]
fn a_select_shows_the_selected_option() {
    let page = layout(
        "<select id=s><option>One</option><option selected>Two</option></select>",
        "",
        800.0,
    );

    assert_eq!(text_of(page.boxed("s")), "Two ▾");
}

#[test]
fn a_textarea_shows_its_text_and_its_rows_set_the_height() {
    let small = layout("<textarea id=t rows=2>abc</textarea>", "", 800.0);
    let tall = layout("<textarea id=t rows=6>abc</textarea>", "", 800.0);

    assert_eq!(text_of(small.boxed("t")), "abc");
    assert!(tall.boxed("t").border_box.size.height > small.boxed("t").border_box.size.height * 2.0);
}

#[test]
fn the_size_attribute_changes_the_width() {
    let narrow = layout("<input id=i size=5>", "", 800.0);
    let wide = layout("<input id=i size=40>", "", 800.0);

    assert!(wide.boxed("i").border_box.size.width > narrow.boxed("i").border_box.size.width * 3.0);
}

#[test]
fn css_width_and_height_win() {
    let page = layout(
        "<input id=i>",
        "input { width: 300px; height: 40px }",
        800.0,
    );

    let field = page.boxed("i").border_box;
    assert_eq!(field.size.width, 300.0);
    assert_eq!(field.size.height, 40.0);
}

#[test]
fn a_block_input_still_renders_as_a_field() {
    let page = layout(
        "<input id=i value='v'>",
        "input { display: block; width: 50% }",
        800.0,
    );

    assert_eq!(text_of(page.boxed("i")), "v");
    assert_eq!(page.boxed("i").border_box.size.width, 392.0);
}

#[test]
fn controls_stack_vertically_with_a_gap() {
    let page = layout("<input id=a><input id=b>", "", 800.0);

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;
    assert!(second.origin.y > first.bottom());
}

#[test]
fn text_after_a_control_comes_below_it() {
    let page = layout("<p id=p>Name <input id=i> after</p>", "", 800.0);

    let field = page.boxed("i").border_box;
    let paragraph = page.boxed("p").border_box;
    assert!(paragraph.bottom() >= field.bottom() - 0.5);
}

#[test]
fn a_focused_text_input_shows_a_caret_and_a_blue_border() {
    let page = layout_focused("<input id=i value='abc'>", "", 800.0, "i", 1);

    let field = page.boxed("i");
    assert_eq!(text_of(field), "a|bc");
    assert_eq!(field.border_color, Rgba::opaque(11, 95, 165));
}

#[test]
fn the_caret_at_the_end_follows_the_text() {
    let page = layout_focused("<input id=i value='abc'>", "", 800.0, "i", 3);

    assert_eq!(text_of(page.boxed("i")), "abc|");
}

#[test]
fn a_focused_empty_input_hides_its_placeholder() {
    let page = layout_focused("<input id=i placeholder='Name'>", "", 800.0, "i", 0);

    assert_eq!(text_of(page.boxed("i")), "|");
}

#[test]
fn only_the_focused_control_changes() {
    let page = layout_focused(
        "<input id=a value='x'><input id=b value='y'>",
        "",
        800.0,
        "a",
        0,
    );

    assert_eq!(text_of(page.boxed("b")), "y");
    assert_eq!(page.boxed("b").border_color, Rgba::opaque(118, 118, 118));
}

#[test]
fn a_long_value_shows_its_tail() {
    let page = layout(
        "<input id=i size=5 value='abcdefghijklmnopqrstuvwxyz'>",
        "",
        800.0,
    );

    let shown = text_of(page.boxed("i"));
    assert!(shown.ends_with('z'));
    assert!(shown.len() < 26);
}

#[test]
fn clicks_inside_a_control_target_it() {
    use fr_geom::geometry::point::Point;
    use fr_layout::hit_test::element_at::element_at;

    let page = layout("<input id=i value='abc'>", "", 800.0);

    let rect = page.boxed("i").border_box;
    let point = Point::new(rect.origin.x + 2.0, rect.origin.y + 2.0);
    assert_eq!(element_at(&page.tree.root, point), Some(page.id("i")));
}

#[test]
fn the_kind_of_each_element_is_recognised() {
    let page = layout(
        "<input id=a type=email><input id=b type=checkbox><input id=c type=image><textarea id=d></textarea><select id=e></select><button id=f></button><p id=g></p><input id=h type=hidden>",
        "",
        800.0,
    );
    let document = &page.document;

    let kind = |name: &str| control_kind_of(document, page.id(name));
    assert_eq!(kind("a"), Some(ControlKind::Text));
    assert_eq!(kind("b"), Some(ControlKind::Checkbox));
    assert_eq!(kind("c"), Some(ControlKind::Button));
    assert_eq!(kind("d"), Some(ControlKind::TextArea));
    assert_eq!(kind("e"), Some(ControlKind::Select));
    assert_eq!(kind("f"), Some(ControlKind::Button));
    assert_eq!(kind("g"), None);
    assert_eq!(kind("h"), None);
}
