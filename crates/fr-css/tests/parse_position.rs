use fr_css::cascade::compute_styles::compute_styles;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_inline_style::parse_inline_style;
use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::value::display::Display;
use fr_css::value::position::Position;
use fr_dom::tree::edge::Edge;
use fr_html::parse::parse_html::parse_html;

fn declarations_of(css: &str) -> Vec<Declaration> {
    parse_inline_style(css)
        .into_iter()
        .map(|entry| entry.declaration)
        .collect()
}

fn display_of(html: &str, css: &str, wanted: &str) -> Display {
    let document = parse_html(html);
    let author = [parse_stylesheet(css)];
    let styles = compute_styles(&document, &user_agent_sheet(), &author);
    let id = document
        .walk(document.root())
        .find_map(|edge| match edge {
            Edge::Enter(id) => {
                let element = document.node(id)?.kind().as_element()?;
                (element.attribute("id") == Some(wanted)).then_some(id)
            }
            Edge::Leave(_) => None,
        })
        .expect("element with the id");
    styles.get(id).expect("style").display
}

#[test]
fn position_values_are_read() {
    for (text, value) in [
        ("static", Position::Static),
        ("relative", Position::Relative),
        ("absolute", Position::Absolute),
        ("fixed", Position::Fixed),
        ("sticky", Position::Sticky),
    ] {
        assert_eq!(
            declarations_of(&format!("position: {text}")),
            vec![Declaration::Position(value)]
        );
    }
    assert!(declarations_of("position: nowhere").is_empty());
}

#[test]
fn only_absolute_and_fixed_leave_the_flow() {
    assert!(Position::Absolute.is_out_of_flow());
    assert!(Position::Fixed.is_out_of_flow());
    assert!(!Position::Static.is_out_of_flow());
    assert!(!Position::Relative.is_out_of_flow());
    assert!(!Position::Sticky.is_out_of_flow());
}

#[test]
fn text_made_visible_to_screen_readers_only_is_not_shown() {
    let css = ".sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0, 0, 0, 0) }";

    assert_eq!(
        display_of("<span id=a class=sr>Toggle menu</span>", css, "a"),
        Display::None
    );
}

#[test]
fn a_tiny_fixed_box_is_hidden_too() {
    assert_eq!(
        display_of(
            "<div id=a style='position: fixed; width: 0; height: 0'>x</div>",
            "",
            "a"
        ),
        Display::None
    );
}

#[test]
fn ordinary_absolute_boxes_and_tiny_static_boxes_stay() {
    assert_eq!(
        display_of(
            "<div id=a style='position: absolute; width: 200px; height: 100px'>x</div>",
            "",
            "a"
        ),
        Display::Block
    );
    assert_eq!(
        display_of("<div id=a style='width: 1px; height: 1px'>x</div>", "", "a"),
        Display::Block
    );
    assert_eq!(
        display_of(
            "<div id=a style='position: absolute; width: 1px'>x</div>",
            "",
            "a"
        ),
        Display::Block
    );
}
