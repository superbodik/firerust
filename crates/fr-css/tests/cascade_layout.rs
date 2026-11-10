use fr_css::cascade::compute_styles::compute_styles;
use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::cascade::style_map::StyleMap;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::value::display::Display;
use fr_css::value::font_family::FontFamily;
use fr_css::value::font_style::FontStyle;
use fr_css::value::length::Length;
use fr_css::value::line_height::LineHeight;
use fr_css::value::text_align::TextAlign;
use fr_css::value::white_space::WhiteSpace;
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

    fn id_of(&self, wanted: &str) -> NodeId {
        self.document
            .walk(self.document.root())
            .find_map(|edge| match edge {
                Edge::Enter(id) => {
                    let element = self.document.node(id)?.kind().as_element()?;
                    (element.attribute("id") == Some(wanted)).then_some(id)
                }
                Edge::Leave(_) => None,
            })
            .expect("element with the id")
    }

    fn style(&self, wanted: &str) -> ComputedStyle {
        *self.styles.get(self.id_of(wanted)).expect("computed style")
    }
}

#[test]
fn text_properties_are_inherited() {
    let page = Styled::new(
        "<div style='text-align: center; font-style: italic; line-height: 2; white-space: pre; font-family: monospace'><span id=a>x</span></div>",
        "",
    );

    let style = page.style("a");
    assert_eq!(style.text_align, TextAlign::Center);
    assert_eq!(style.font_style, FontStyle::Italic);
    assert_eq!(style.line_height, LineHeight::Number(2.0));
    assert_eq!(style.white_space, WhiteSpace::Pre);
    assert_eq!(style.font_family, FontFamily::Monospace);
}

#[test]
fn sizes_and_box_sizing_are_not_inherited() {
    let page = Styled::new(
        "<div style='width: 100px; box-sizing: border-box'><span id=a>x</span></div>",
        "",
    );

    assert_eq!(page.style("a").size.width, Length::Auto);
    assert_ne!(
        page.style("a").box_sizing,
        fr_css::value::box_sizing::BoxSizing::BorderBox
    );
}

#[test]
fn em_sizes_resolve_against_the_font_size() {
    let page = Styled::new(
        "<p id=a style='font-size: 20px; width: 10em; max-width: 2em'>x</p>",
        "",
    );

    assert_eq!(page.style("a").size.width, Length::Px(200.0));
    assert_eq!(page.style("a").size.max_width, Length::Px(40.0));
}

#[test]
fn percent_sizes_stay_percent() {
    let page = Styled::new("<div id=a style='width: 50%'>x</div>", "");

    assert_eq!(page.style("a").size.width, Length::Percent(50.0));
}

#[test]
fn flex_properties_are_computed() {
    let page = Styled::new(
        "<div id=a style='display: flex; gap: 1em; font-size: 10px'><p id=b style='flex: 2'>x</p></div>",
        "",
    );

    let container = page.style("a");
    assert_eq!(container.display, Display::Flex);
    assert_eq!(container.flex.row_gap, 10.0);
    assert_eq!(container.flex.column_gap, 10.0);
    let item = page.style("b");
    assert_eq!(item.flex.grow, 2.0);
    assert_eq!(item.flex.shrink, 1.0);
    assert_eq!(item.flex.basis, Length::Px(0.0));
}

#[test]
fn tables_have_their_default_displays() {
    let page = Styled::new(
        "<table id=t><thead id=h><tr id=r><th id=th>a</th></tr></thead><tbody><tr><td id=td>b</td></tr></tbody></table>",
        "",
    );

    assert_eq!(page.style("t").display, Display::Table);
    assert_eq!(page.style("h").display, Display::TableRowGroup);
    assert_eq!(page.style("r").display, Display::TableRow);
    assert_eq!(page.style("th").display, Display::TableCell);
    assert_eq!(page.style("td").display, Display::TableCell);
    assert_eq!(page.style("td").padding.left, Length::Px(1.0));
}

#[test]
fn pre_and_code_have_their_defaults() {
    let page = Styled::new("<pre id=p><code id=c>x</code></pre><em id=e>y</em>", "");

    assert_eq!(page.style("p").white_space, WhiteSpace::Pre);
    assert_eq!(page.style("c").font_family, FontFamily::Monospace);
    assert_eq!(page.style("e").font_style, FontStyle::Italic);
}

#[test]
fn the_bgcolor_attribute_sets_a_background() {
    let page = Styled::new(
        "<table id=a bgcolor='#ff6600'><tr><td>x</td></tr></table>",
        "",
    );

    assert_eq!(page.style("a").background_color, Rgba::opaque(255, 102, 0));
}

#[test]
fn a_css_background_beats_the_bgcolor_attribute() {
    let page = Styled::new(
        "<table id=a bgcolor='#ff6600'><tr><td>x</td></tr></table>",
        "table { background-color: #000000 }",
    );

    assert_eq!(page.style("a").background_color, Rgba::opaque(0, 0, 0));
}

#[test]
fn the_font_color_attribute_sets_the_text_color() {
    let page = Styled::new("<p><font id=a color='#828282'>x</font></p>", "");

    assert_eq!(page.style("a").color, Rgba::opaque(0x82, 0x82, 0x82));
}

#[test]
fn a_bare_hex_attribute_value_is_accepted() {
    let page = Styled::new("<div id=a bgcolor='f60'>x</div>", "");

    assert_eq!(page.style("a").background_color, Rgba::opaque(255, 102, 0));
}

#[test]
fn a_table_resets_the_text_alignment_of_a_center_element() {
    let page = Styled::new(
        "<center><table id=t><tr><td id=c>x</td></tr></table><p id=p>y</p></center>",
        "",
    );

    assert_eq!(page.style("c").text_align, TextAlign::Left);
    assert_eq!(page.style("p").text_align, TextAlign::Center);
}

#[test]
fn a_table_inside_a_center_element_gets_auto_margins() {
    let page = Styled::new(
        "<center><table id=t><tr><td>x</td></tr></table></center>",
        "",
    );

    assert_eq!(page.style("t").margin.left, Length::Auto);
    assert_eq!(page.style("t").margin.right, Length::Auto);
}
