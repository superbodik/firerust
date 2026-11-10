use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_form::state::attribute_of::attribute_of;
use fr_form::state::choose_radio::choose_radio;
use fr_form::state::cycle_select::cycle_select;
use fr_form::state::focusable_controls::focusable_controls;
use fr_form::state::neighbour_control::neighbour_control;
use fr_form::state::read_value::read_value;
use fr_form::state::toggle_checkbox::toggle_checkbox;
use fr_form::state::write_value::write_value;
use fr_html::parse::parse_html::parse_html;

fn find(document: &Document, wanted: &str) -> NodeId {
    document
        .walk(document.root())
        .find_map(|edge| match edge {
            Edge::Enter(id) => {
                let element = document.node(id)?.kind().as_element()?;
                (element.attribute("id") == Some(wanted)).then_some(id)
            }
            Edge::Leave(_) => None,
        })
        .expect("element with the id")
}

#[test]
fn an_input_value_is_read_from_its_attribute() {
    let document = parse_html("<input id=i value='abc'>");

    assert_eq!(read_value(&document, find(&document, "i")), "abc");
}

#[test]
fn a_textarea_value_is_its_text() {
    let document = parse_html("<textarea id=t>hi there</textarea>");

    assert_eq!(read_value(&document, find(&document, "t")), "hi there");
}

#[test]
fn writing_an_input_sets_its_attribute() {
    let mut document = parse_html("<input id=i value='abc'>");
    let input = find(&document, "i");

    write_value(&mut document, input, "changed").unwrap();

    assert_eq!(read_value(&document, input), "changed");
    assert_eq!(
        attribute_of(&document, input, "value").as_deref(),
        Some("changed")
    );
}

#[test]
fn writing_a_textarea_replaces_its_text() {
    let mut document = parse_html("<textarea id=t>old</textarea>");
    let area = find(&document, "t");

    write_value(&mut document, area, "new\nlines").unwrap();

    assert_eq!(read_value(&document, area), "new\nlines");
}

#[test]
fn toggling_a_checkbox_flips_its_state() {
    let mut document = parse_html("<input id=c type=checkbox>");
    let checkbox = find(&document, "c");

    assert!(toggle_checkbox(&mut document, checkbox).unwrap());
    assert!(attribute_of(&document, checkbox, "checked").is_some());
    assert!(!toggle_checkbox(&mut document, checkbox).unwrap());
    assert!(attribute_of(&document, checkbox, "checked").is_none());
}

#[test]
fn choosing_a_radio_clears_the_others_with_the_same_name_in_its_form() {
    let mut document = parse_html(
        "<form><input id=a type=radio name=r checked><input id=b type=radio name=r></form><form><input id=c type=radio name=r checked></form>",
    );

    let second = find(&document, "b");
    choose_radio(&mut document, second).unwrap();

    assert!(attribute_of(&document, find(&document, "a"), "checked").is_none());
    assert!(attribute_of(&document, find(&document, "b"), "checked").is_some());
    assert!(attribute_of(&document, find(&document, "c"), "checked").is_some());
}

#[test]
fn cycling_a_select_moves_to_the_next_option_and_wraps() {
    let mut document = parse_html(
        "<select id=s><option id=o1>A</option><option id=o2>B</option><option id=o3>C</option></select>",
    );
    let select = find(&document, "s");

    cycle_select(&mut document, select).unwrap();
    assert!(attribute_of(&document, find(&document, "o2"), "selected").is_some());
    cycle_select(&mut document, select).unwrap();
    cycle_select(&mut document, select).unwrap();

    assert!(attribute_of(&document, find(&document, "o1"), "selected").is_some());
    assert!(attribute_of(&document, find(&document, "o3"), "selected").is_none());
}

#[test]
fn focusable_controls_follow_document_order_and_skip_disabled_and_hidden() {
    let document = parse_html(
        "<input id=a><input id=b type=hidden><input id=c disabled><textarea id=d></textarea><p id=p></p><button id=e>x</button>",
    );

    let controls = focusable_controls(&document);

    assert_eq!(
        controls,
        vec![
            find(&document, "a"),
            find(&document, "d"),
            find(&document, "e")
        ]
    );
}

#[test]
fn tab_moves_forward_and_wraps_around() {
    let document = parse_html("<input id=a><input id=b><input id=c>");
    let controls = focusable_controls(&document);
    let (a, b, c) = (
        find(&document, "a"),
        find(&document, "b"),
        find(&document, "c"),
    );

    assert_eq!(neighbour_control(&controls, Some(a), true), Some(b));
    assert_eq!(neighbour_control(&controls, Some(c), true), Some(a));
}

#[test]
fn shift_tab_moves_backward_and_wraps_around() {
    let document = parse_html("<input id=a><input id=b><input id=c>");
    let controls = focusable_controls(&document);
    let (a, b, c) = (
        find(&document, "a"),
        find(&document, "b"),
        find(&document, "c"),
    );

    assert_eq!(neighbour_control(&controls, Some(b), false), Some(a));
    assert_eq!(neighbour_control(&controls, Some(a), false), Some(c));
}

#[test]
fn without_a_current_control_tab_picks_the_first_and_shift_tab_the_last() {
    let document = parse_html("<input id=a><input id=b>");
    let controls = focusable_controls(&document);

    assert_eq!(
        neighbour_control(&controls, None, true),
        Some(find(&document, "a"))
    );
    assert_eq!(
        neighbour_control(&controls, None, false),
        Some(find(&document, "b"))
    );
}

#[test]
fn a_page_without_controls_has_nothing_to_focus() {
    assert_eq!(neighbour_control(&[], None, true), None);
}
