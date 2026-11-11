mod support;

use support::layout;

const TEXT: &str = "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor";

fn near(actual: f32, expected: f32) -> bool {
    (actual - expected).abs() < 1.5
}

#[test]
fn a_left_float_sits_at_the_left_edge_and_the_next_block_moves_beside_it() {
    let page = layout(
        &format!(
            "<div id=f style='float: left; width: 200px; height: 100px'></div><p id=p>{TEXT}</p>"
        ),
        "",
        800.0,
    );

    let float = page.boxed("f").border_box;
    let paragraph = page.boxed("p").border_box;

    assert!(near(float.origin.x, 8.0));
    assert!(near(float.size.width, 200.0));
    assert!(near(paragraph.origin.x, 208.0));
    assert!(near(paragraph.size.width, 584.0));
}

#[test]
fn a_right_float_sits_against_the_right_edge() {
    let page = layout(
        &format!(
            "<div id=f style='float: right; width: 150px; height: 60px'></div><p id=p>{TEXT}</p>"
        ),
        "",
        800.0,
    );

    let float = page.boxed("f").border_box;
    let paragraph = page.boxed("p").border_box;

    assert!(near(float.right(), 792.0));
    assert!(near(paragraph.origin.x, 8.0));
    assert!(near(paragraph.size.width, 634.0));
}

#[test]
fn content_below_the_float_uses_the_full_width_again() {
    let page = layout(
        "<div style='float: left; width: 120px; height: 30px'></div>\
         <div id=a style='height: 60px'></div><div id=b style='height: 10px'></div>",
        "",
        800.0,
    );

    let beside = page.boxed("a").border_box;
    let below = page.boxed("b").border_box;

    assert!(near(beside.origin.x, 128.0));
    assert!(near(below.origin.x, 8.0));
    assert!(near(below.size.width, 784.0));
}

#[test]
fn clear_moves_a_block_below_the_floats() {
    let page = layout(
        "<div id=f style='float: left; width: 100px; height: 100px'></div>\
         <div id=c style='clear: both; height: 10px'></div>",
        "",
        800.0,
    );

    let float = page.boxed("f").border_box;
    let cleared = page.boxed("c").border_box;

    assert!(cleared.origin.y >= float.bottom() - 0.5);
    assert!(near(cleared.origin.x, 8.0));
    assert!(near(cleared.size.width, 784.0));
}

#[test]
fn clear_left_ignores_a_right_float() {
    let page = layout(
        "<div id=f style='float: right; width: 100px; height: 100px'></div>\
         <div id=c style='clear: left; height: 10px'></div>",
        "",
        800.0,
    );

    let cleared = page.boxed("c").border_box;

    assert!(cleared.origin.y < 50.0);
}

#[test]
fn two_left_floats_stand_side_by_side() {
    let page = layout(
        "<div id=a style='float: left; width: 100px; height: 50px'></div>\
         <div id=b style='float: left; width: 120px; height: 50px'></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;

    assert!(near(second.origin.x, first.right()));
    assert!(near(second.origin.y, first.origin.y));
}

#[test]
fn a_float_that_does_not_fit_beside_another_drops_below_it() {
    let page = layout(
        "<div id=a style='float: left; width: 500px; height: 40px'></div>\
         <div id=b style='float: left; width: 500px; height: 40px'></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;

    assert!(second.origin.y >= first.bottom() - 0.5);
    assert!(near(second.origin.x, 8.0));
}

#[test]
fn a_container_grows_to_hold_its_floats() {
    let page = layout(
        "<div id=wrap><div style='float: left; width: 50px; height: 200px'></div></div>\
         <div id=after style='height: 10px'></div>",
        "",
        800.0,
    );

    let wrap = page.boxed("wrap").border_box;
    let after = page.boxed("after").border_box;

    assert!(wrap.size.height >= 199.0);
    assert!(after.origin.y >= wrap.bottom() - 0.5);
}

#[test]
fn a_float_without_a_width_shrinks_to_its_content() {
    let page = layout("<div id=f style='float: left'>short</div>", "", 800.0);

    let float = page.boxed("f").border_box;

    assert!(float.size.width > 10.0);
    assert!(float.size.width < 120.0);
}

#[test]
fn a_float_keeps_its_margins() {
    let page = layout(
        "<div id=f style='float: left; width: 100px; height: 20px; margin: 10px 15px'></div>\
         <p id=p>text</p>",
        "",
        800.0,
    );

    let float = page.boxed("f").border_box;
    let paragraph = page.boxed("p").border_box;

    assert!(near(float.origin.x, 23.0));
    assert!(near(paragraph.origin.x, 8.0 + 15.0 + 100.0 + 15.0));
}

#[test]
fn float_none_and_hidden_floats_change_nothing() {
    let none = layout(
        "<div style='float: none; width: 100px; height: 40px'></div><div id=b style='height: 10px'></div>",
        "",
        800.0,
    );
    let hidden = layout(
        "<div style='float: left; display: none; width: 100px; height: 40px'></div><div id=b style='height: 10px'></div>",
        "",
        800.0,
    );

    assert!(near(none.boxed("b").border_box.origin.x, 8.0));
    assert!(near(hidden.boxed("b").border_box.origin.x, 8.0));
    assert!(none.boxed("b").border_box.origin.y >= 40.0);
    assert!(hidden.boxed("b").border_box.origin.y < 20.0);
}

#[test]
fn the_float_property_is_not_inherited() {
    let page = layout(
        "<div style='float: left; width: 300px'><div id=inner style='height: 10px'>x</div></div>",
        "",
        800.0,
    );

    assert!(near(page.boxed("inner").border_box.origin.x, 8.0));
}

#[test]
fn a_float_wider_than_a_narrow_container_does_not_hang() {
    let page = layout(
        &format!(
            "<div id=box style='width: 200px'><div id=f style='float: left; width: 300px; height: 20px'></div><p id=p>{TEXT}</p></div>"
        ),
        "",
        800.0,
    );

    assert!(page.boxed("f").border_box.size.width > 0.0);
    assert!(page.boxed("p").border_box.size.height > 0.0);
}

#[test]
fn text_beside_a_float_wraps_in_the_narrower_column() {
    let narrow = layout(
        &format!(
            "<div style='float: left; width: 400px; height: 400px'></div><p id=p>{TEXT} {TEXT} {TEXT}</p>"
        ),
        "",
        800.0,
    );
    let plain = layout(&format!("<p id=p>{TEXT} {TEXT} {TEXT}</p>"), "", 800.0);

    assert!(narrow.boxed("p").border_box.size.height > plain.boxed("p").border_box.size.height);
}

#[test]
fn text_is_never_squeezed_into_a_sliver_beside_a_wide_float() {
    let page = layout(
        &format!(
            "<div id=box style='width: 300px'><div style='float: left; width: 280px; height: 60px'></div><p id=p>{TEXT}</p></div>"
        ),
        "",
        800.0,
    );

    let paragraph = page.boxed("p").border_box;

    assert!(paragraph.size.width >= 100.0 || paragraph.origin.y >= 60.0);
}

#[test]
fn a_narrow_column_clears_the_float_instead_of_collapsing() {
    let page = layout(
        "<div style='width: 120px'><div id=f style='float: right; width: 100px; height: 80px'></div>\
         <div id=b style='height: 10px'></div></div>",
        "",
        800.0,
    );

    let float = page.boxed("f").border_box;
    let below = page.boxed("b").border_box;

    assert!(below.origin.y >= float.bottom() - 0.5);
}
