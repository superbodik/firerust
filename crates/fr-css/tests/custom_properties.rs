use fr_css::cascade::compute_styles::compute_styles;
use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::cascade::custom_properties::CustomProperties;
use fr_css::cascade::style_map::StyleMap;
use fr_css::cascade::substitute_vars::substitute_vars;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_geom::color::rgba::Rgba;
use fr_html::parse::parse_html::parse_html;

struct Styled {
    document: Document,
    styles: StyleMap,
}

impl Styled {
    fn new(html: &str, css: &str) -> Self {
        let document = parse_html(html);
        let author = [parse_stylesheet(css)];
        let styles = compute_styles(&document, &user_agent_sheet(), &author);
        Self { document, styles }
    }

    fn style(&self, wanted: &str) -> ComputedStyle {
        let id: NodeId = self
            .document
            .walk(self.document.root())
            .find_map(|edge| match edge {
                Edge::Enter(id) => {
                    let element = self.document.node(id)?.kind().as_element()?;
                    (element.attribute("id") == Some(wanted)).then_some(id)
                }
                Edge::Leave(_) => None,
            })
            .expect("element with the id");
        *self.styles.get(id).expect("computed style")
    }
}

const RED: Rgba = Rgba::opaque(255, 0, 0);
const BLUE: Rgba = Rgba::opaque(0, 0, 255);

fn vars(pairs: &[(&str, &str)]) -> CustomProperties {
    pairs
        .iter()
        .map(|(name, value)| ((*name).into(), (*value).into()))
        .collect()
}

#[test]
fn a_variable_defined_on_root_reaches_every_element() {
    let page = Styled::new(
        "<div id=a><p id=b>x</p></div>",
        ":root { --brand: #ff0000 } p { color: var(--brand) }",
    );

    assert_eq!(page.style("b").color, RED);
}

#[test]
fn a_variable_set_on_an_element_is_inherited_by_its_children() {
    let page = Styled::new(
        "<div id=a><p id=b>x</p></div><p id=c>y</p>",
        "div { --tone: blue } p { color: var(--tone, red) }",
    );

    assert_eq!(page.style("b").color, BLUE);
    assert_eq!(page.style("c").color, RED);
}

#[test]
fn a_child_can_override_the_value_for_its_own_subtree() {
    let page = Styled::new(
        "<div id=a><p id=b>x</p></div><div id=c class=other><p id=d>y</p></div>",
        ":root { --tone: red } .other { --tone: blue } p { color: var(--tone) }",
    );

    assert_eq!(page.style("b").color, RED);
    assert_eq!(page.style("d").color, BLUE);
}

#[test]
fn the_fallback_is_used_when_the_variable_is_missing() {
    let page = Styled::new("<p id=a>x</p>", "p { color: var(--nothing, blue) }");

    assert_eq!(page.style("a").color, BLUE);
}

#[test]
fn a_missing_variable_without_a_fallback_drops_the_declaration() {
    let page = Styled::new("<p id=a>x</p>", "p { color: red; color: var(--nothing) }");

    assert_eq!(page.style("a").color, RED);
}

#[test]
fn variables_work_inside_shorthands_and_lengths() {
    let page = Styled::new(
        "<div id=a>x</div>",
        ":root { --gap: 12px; --edge: 3px solid #0000ff } \
         div { margin: var(--gap) 4px; border: var(--edge); padding: calc-free }",
    );

    let style = page.style("a");
    assert_eq!(style.margin.top, Length::Px(12.0));
    assert_eq!(style.margin.left, Length::Px(4.0));
    assert_eq!(style.border_width.top, 3.0);
    assert_eq!(style.border_color, BLUE);
}

#[test]
fn a_variable_can_refer_to_another_variable() {
    let page = Styled::new(
        "<p id=a>x</p>",
        ":root { --base: #0000ff; --accent: var(--base) } p { color: var(--accent) }",
    );

    assert_eq!(page.style("a").color, BLUE);
}

#[test]
fn the_cascade_order_still_decides_between_variable_and_plain_values() {
    let page = Styled::new(
        "<p id=a>x</p>",
        ":root { --tone: blue } p { color: var(--tone) } p { color: red }",
    );

    assert_eq!(page.style("a").color, RED);
}

#[test]
fn an_inline_style_can_use_variables() {
    let page = Styled::new(
        "<p id=a style='color: var(--tone)'>x</p>",
        ":root { --tone: blue }",
    );

    assert_eq!(page.style("a").color, BLUE);
}

#[test]
fn important_is_kept_on_a_variable_declaration() {
    let page = Styled::new(
        "<p id=a>x</p>",
        ":root { --tone: blue } p { color: var(--tone) !important } p { color: red }",
    );

    assert_eq!(page.style("a").color, BLUE);
}

#[test]
fn substitution_handles_nesting_names_and_cycles() {
    let table = vars(&[
        ("--a", "1px"),
        ("--b", "var(--a)"),
        ("--loop", "var(--loop)"),
    ]);

    assert_eq!(
        substitute_vars("var(--a) var(--b)", &table).as_deref(),
        Some("1px 1px")
    );
    assert_eq!(
        substitute_vars("var(--x, var(--a))", &table).as_deref(),
        Some("1px")
    );
    assert_eq!(
        substitute_vars("calc(var(--a) + 2px)", &table).as_deref(),
        Some("calc(1px + 2px)")
    );
    assert_eq!(substitute_vars("var(--x)", &table), None);
    assert_eq!(substitute_vars("var(--loop)", &table), None);
    assert_eq!(substitute_vars("var(--a", &table), None);
    assert_eq!(substitute_vars("VAR(--a)", &table).as_deref(), Some("1px"));
}
