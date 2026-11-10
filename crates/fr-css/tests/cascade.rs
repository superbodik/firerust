use fr_css::cascade::compute_styles::compute_styles;
use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::cascade::style_map::StyleMap;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::value::display::Display;
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

const RED: Rgba = Rgba::opaque(255, 0, 0);
const BLUE: Rgba = Rgba::opaque(0, 0, 255);
const GREEN: Rgba = Rgba::opaque(0, 128, 0);

#[test]
fn an_id_beats_a_class_and_a_tag() {
    let page = Styled::new(
        "<p id=a class=b>x</p>",
        "p { color: red } .b { color: green } #a { color: blue }",
    );

    assert_eq!(page.style("a").color, BLUE);
}

#[test]
fn a_class_beats_a_tag_even_when_written_earlier() {
    let page = Styled::new(
        "<p id=a class=b>x</p>",
        ".b { color: green } p { color: red }",
    );

    assert_eq!(page.style("a").color, GREEN);
}

#[test]
fn the_later_rule_wins_at_equal_specificity() {
    let page = Styled::new("<p id=a>x</p>", "p { color: red } p { color: blue }");

    assert_eq!(page.style("a").color, BLUE);
}

#[test]
fn important_beats_specificity() {
    let page = Styled::new(
        "<p id=a class=b>x</p>",
        "p { color: red !important } #a { color: blue }",
    );

    assert_eq!(page.style("a").color, RED);
}

#[test]
fn an_inline_style_beats_an_id_rule() {
    let page = Styled::new("<p id=a style='color: green'>x</p>", "#a { color: blue }");

    assert_eq!(page.style("a").color, GREEN);
}

#[test]
fn an_important_rule_beats_a_plain_inline_style() {
    let page = Styled::new(
        "<p id=a style='color: green'>x</p>",
        "p { color: red !important }",
    );

    assert_eq!(page.style("a").color, RED);
}

#[test]
fn author_rules_beat_the_user_agent_sheet() {
    let page = Styled::new("<a id=a href='/'>x</a>", "a { color: green }");

    assert_eq!(page.style("a").color, GREEN);
}

#[test]
fn color_and_font_weight_are_inherited() {
    let page = Styled::new(
        "<div style='color: red; font-weight: bold'><span id=a>x</span></div>",
        "",
    );

    assert_eq!(page.style("a").color, RED);
    assert_eq!(page.style("a").font_weight, 700);
}

#[test]
fn background_and_margin_are_not_inherited() {
    let page = Styled::new(
        "<div style='background-color: red; margin-left: 9px'><span id=a>x</span></div>",
        "",
    );

    assert_eq!(page.style("a").background_color.a, 0);
    assert_eq!(page.style("a").margin.left, Length::Px(0.0));
}

#[test]
fn em_sizes_follow_the_parent_font_size() {
    let page = Styled::new(
        "<div style='font-size: 20px'><p id=a style='font-size: 2em; margin-left: 1em'>x</p></div>",
        "",
    );

    assert_eq!(page.style("a").font_size, 40.0);
    assert_eq!(page.style("a").margin.left, Length::Px(40.0));
}

#[test]
fn percent_font_sizes_follow_the_parent_font_size() {
    let page = Styled::new(
        "<div style='font-size: 20px'><span id=a style='font-size: 150%'>x</span></div>",
        "",
    );

    assert_eq!(page.style("a").font_size, 30.0);
}

#[test]
fn rem_sizes_follow_the_root() {
    let page = Styled::new("<p id=a style='font-size: 2rem'>x</p>", "");

    assert_eq!(page.style("a").font_size, 32.0);
}

#[test]
fn a_zero_font_size_is_kept_above_zero() {
    let page = Styled::new("<p id=a style='font-size: 0'>x</p>", "");

    assert!(page.style("a").font_size >= 1.0);
}

#[test]
fn headings_are_big_bold_blocks() {
    let page = Styled::new("<h1 id=a>x</h1>", "");

    assert_eq!(page.style("a").display, Display::Block);
    assert_eq!(page.style("a").font_size, 32.0);
    assert_eq!(page.style("a").font_weight, 700);
    assert!((page.style("a").margin.top.px_or_zero() - 21.44).abs() < 0.01);
}

#[test]
fn head_content_is_not_displayed() {
    let page = Styled::new(
        "<html><head><title id=t>x</title><style id=s>p{}</style></head><body><script id=c></script><span id=i>x</span></body></html>",
        "",
    );

    assert_eq!(page.style("t").display, Display::None);
    assert_eq!(page.style("s").display, Display::None);
    assert_eq!(page.style("c").display, Display::None);
    assert_eq!(page.style("i").display, Display::Inline);
}

#[test]
fn list_items_and_lists_have_their_defaults() {
    let page = Styled::new("<ul id=u><li id=l>x</li></ul>", "");

    assert_eq!(page.style("l").display, Display::ListItem);
    assert_eq!(page.style("u").padding.left, Length::Px(40.0));
}

#[test]
fn shorthands_expand_to_each_side() {
    let page = Styled::new(
        "<div id=a style='margin: 1px 2px 3px 4px; padding: 5px'>x</div>",
        "",
    );

    let style = page.style("a");
    assert_eq!(style.margin.top, Length::Px(1.0));
    assert_eq!(style.margin.right, Length::Px(2.0));
    assert_eq!(style.margin.bottom, Length::Px(3.0));
    assert_eq!(style.margin.left, Length::Px(4.0));
    assert_eq!(style.padding.bottom, Length::Px(5.0));
}

#[test]
fn a_border_takes_its_color_from_the_text_when_unspecified() {
    let page = Styled::new(
        "<div id=a style='color: red; border: 2px solid'>x</div>",
        "",
    );

    assert_eq!(page.style("a").border_width.left, 2.0);
    assert_eq!(page.style("a").border_color, RED);
}

#[test]
fn descendant_and_child_rules_apply() {
    let page = Styled::new(
        "<div class=box><p id=a>x</p><section><p id=b>y</p></section></div>",
        ".box > p { color: red } .box p { font-weight: bold }",
    );

    assert_eq!(page.style("a").color, RED);
    assert_eq!(page.style("b").color, Rgba::BLACK);
    assert_eq!(page.style("a").font_weight, 700);
    assert_eq!(page.style("b").font_weight, 700);
}

#[test]
fn display_none_from_a_rule_is_applied() {
    let page = Styled::new("<p id=a class=hidden>x</p>", ".hidden { display: none }");

    assert_eq!(page.style("a").display, Display::None);
}

#[test]
fn an_invalid_declaration_leaves_the_default() {
    let page = Styled::new("<p id=a>x</p>", "p { color: banana }");

    assert_eq!(page.style("a").color, Rgba::BLACK);
}
