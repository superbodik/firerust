mod support;

use fr_geom::color::rgba::Rgba;
use support::layout;

#[test]
fn cells_of_a_row_sit_side_by_side() {
    let page = layout(
        "<table><tr><td id=a>one</td><td id=b>two</td></tr></table>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;
    assert_eq!(first.origin.y, second.origin.y);
    assert!((second.origin.x - first.right()).abs() < 0.01);
    assert!(first.size.width > 0.0 && second.size.width > 0.0);
}

#[test]
fn columns_line_up_across_rows() {
    let page = layout(
        "<table><tr><td id=a1>a</td><td id=b1>longer second column</td></tr><tr><td id=a2>a much longer first column</td><td id=b2>b</td></tr></table>",
        "",
        800.0,
    );

    assert_eq!(
        page.boxed("a1").border_box.origin.x,
        page.boxed("a2").border_box.origin.x
    );
    assert_eq!(
        page.boxed("b1").border_box.origin.x,
        page.boxed("b2").border_box.origin.x
    );
    assert_eq!(
        page.boxed("a1").border_box.size.width,
        page.boxed("a2").border_box.size.width
    );
}

#[test]
fn rows_stack_downwards() {
    let page = layout(
        "<table><tr id=r1><td>a</td></tr><tr id=r2><td>b</td></tr></table>",
        "",
        800.0,
    );

    let first = page.boxed("r1").border_box;
    let second = page.boxed("r2").border_box;
    assert_eq!(second.origin.y, first.bottom());
}

#[test]
fn a_row_is_as_tall_as_its_tallest_cell() {
    let page = layout(
        "<table><tr id=r><td id=a style='height: 10px'>x</td><td id=b>one two three four five six seven eight nine ten eleven twelve thirteen</td></tr></table>",
        "td { width: 120px }",
        800.0,
    );

    let row = page.boxed("r").border_box;
    assert!(row.size.height > 30.0);
    assert_eq!(page.boxed("a").border_box.size.height, row.size.height);
    assert_eq!(page.boxed("b").border_box.size.height, row.size.height);
}

#[test]
fn a_table_is_as_narrow_as_its_content_by_default() {
    let page = layout("<table id=t><tr><td>short</td></tr></table>", "", 800.0);

    assert!(page.boxed("t").border_box.size.width < 100.0);
}

#[test]
fn a_full_width_table_fills_its_container() {
    let page = layout(
        "<table id=t style='width: 100%'><tr><td id=a>a</td><td id=b>b</td></tr></table>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("t").border_box.size.width, 784.0);
    let total = page.boxed("a").border_box.size.width + page.boxed("b").border_box.size.width;
    assert!((total - 784.0).abs() < 0.01);
}

#[test]
fn the_width_attribute_sets_the_table_width() {
    let page = layout(
        "<table id=t width=300><tr><td>a</td></tr></table>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("t").border_box.size.width, 300.0);
}

#[test]
fn a_percent_width_attribute_is_relative_to_the_container() {
    let page = layout(
        "<table id=t width='50%'><tr><td>a</td></tr></table>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("t").border_box.size.width, 392.0);
}

#[test]
fn a_colspan_cell_covers_the_columns_it_spans() {
    let page = layout(
        "<table><tr><td id=a>one</td><td id=b>two</td></tr><tr><td id=c colspan=2>spanning</td></tr></table>",
        "",
        800.0,
    );

    let span = page.boxed("c").border_box;
    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;
    assert_eq!(span.origin.x, first.origin.x);
    assert!((span.size.width - (first.size.width + second.size.width)).abs() < 0.01);
}

#[test]
fn a_wide_spanning_cell_widens_its_columns() {
    let page = layout(
        "<table id=t><tr><td id=a>a</td><td id=b>b</td></tr><tr><td colspan=2>a long spanning sentence that needs room</td></tr></table>",
        "",
        800.0,
    );

    let columns = page.boxed("a").border_box.size.width + page.boxed("b").border_box.size.width;
    assert!(columns > 150.0, "columns are {columns}");
}

#[test]
fn row_groups_are_flattened_in_order() {
    let page = layout(
        "<table><thead><tr id=h><td>head</td></tr></thead><tbody><tr id=b1><td>one</td></tr><tr id=b2><td>two</td></tr></tbody></table>",
        "",
        800.0,
    );

    let head = page.boxed("h").border_box.origin.y;
    let one = page.boxed("b1").border_box.origin.y;
    let two = page.boxed("b2").border_box.origin.y;
    assert!(head < one && one < two);
}

#[test]
fn cell_padding_and_borders_are_part_of_the_cell() {
    let page = layout(
        "<table><tr><td id=a style='padding: 10px; border: 2px solid red'>x</td></tr></table>",
        "",
        800.0,
    );

    let cell = page.boxed("a");
    assert_eq!(cell.border_width.left, 2.0);
    assert!(cell.border_box.size.width >= 24.0);
    assert!(cell.border_box.size.height >= 24.0);
}

#[test]
fn row_and_cell_backgrounds_are_kept() {
    let page = layout(
        "<table><tr id=r bgcolor='#ff6600'><td id=a style='background-color: #0000ff'>x</td></tr></table>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("r").background, Rgba::opaque(255, 102, 0));
    assert_eq!(page.boxed("a").background, Rgba::opaque(0, 0, 255));
}

#[test]
fn text_wraps_in_a_narrow_table() {
    let narrow = layout(
        "<table id=t width=120><tr><td>the quick brown fox jumps over the lazy dog</td></tr></table>",
        "",
        800.0,
    );
    let wide = layout(
        "<table id=t width=600><tr><td>the quick brown fox jumps over the lazy dog</td></tr></table>",
        "",
        800.0,
    );

    assert!(
        narrow.boxed("t").border_box.size.height > wide.boxed("t").border_box.size.height * 2.0
    );
}

#[test]
fn a_table_can_be_centered_with_auto_margins() {
    let page = layout(
        "<table id=t width=200 style='margin: 0 auto'><tr><td>x</td></tr></table>",
        "",
        816.0,
    );

    assert_eq!(
        page.boxed("t").border_box.origin.x,
        8.0 + (800.0 - 200.0) / 2.0
    );
}

#[test]
fn tables_nest_inside_cells() {
    let page = layout(
        "<table id=outer><tr><td id=cell><table id=inner><tr><td id=x>deep</td></tr></table></td></tr></table>",
        "",
        800.0,
    );

    let cell = page.boxed("cell").border_box;
    let inner = page.boxed("inner").border_box;
    assert!(inner.origin.x >= cell.origin.x);
    assert!(inner.right() <= cell.right() + 0.5);
    assert!(page.boxed("x").border_box.size.width > 0.0);
}

#[test]
fn a_table_is_a_block_for_the_content_after_it() {
    let page = layout(
        "<table id=t><tr><td>a</td></tr></table><div id=next style='height: 5px'></div>",
        "",
        800.0,
    );

    assert_eq!(
        page.boxed("next").border_box.origin.y,
        page.boxed("t").border_box.bottom()
    );
}

#[test]
fn an_empty_table_has_no_height() {
    let page = layout("<table id=t></table>", "", 800.0);

    assert_eq!(page.boxed("t").border_box.size.height, 0.0);
}

#[test]
fn a_table_inside_a_flex_item_gets_its_content_width() {
    let page = layout(
        "<div style='display: flex'><div id=a><table id=t><tr><td>some cell text</td></tr></table></div><div id=b style='width: 50px; height: 5px'></div></div>",
        "",
        800.0,
    );

    let item = page.boxed("a").border_box;
    assert!(item.size.width > 50.0 && item.size.width < 300.0);
    assert!(page.boxed("t").border_box.right() <= item.right() + 0.5);
}
