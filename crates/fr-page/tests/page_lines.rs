use fr_css::cascade::compute_styles::compute_styles;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_geom::color::rgba::Rgba;
use fr_html::parse::parse_html::parse_html;
use fr_page::lines::backdrop::Backdrop;
use fr_page::lines::canvas_background::canvas_background;
use fr_page::lines::line::Line;
use fr_page::lines::page_lines::page_lines;
use fr_page::style::collect_author_sheets::collect_author_sheets;
use fr_page::text::document_title::document_title;
use url::Url;

fn lines_of(html: &str) -> Vec<Line> {
    let document = parse_html(html);
    let base = Url::parse("file:///page.html").expect("url");
    let author = collect_author_sheets(&document, &base);
    let styles = compute_styles(&document, &user_agent_sheet(), &author);
    page_lines(&document, &styles)
}

fn texts_of(html: &str) -> Vec<String> {
    lines_of(html).iter().map(Line::plain_text).collect()
}

fn gaps_of(html: &str) -> Vec<f32> {
    lines_of(html).iter().map(|line| line.gap_before).collect()
}

#[test]
fn inline_tags_do_not_break_lines() {
    assert_eq!(texts_of("<p>Hello <b>world</b>!</p>"), vec!["Hello world!"]);
}

#[test]
fn inline_tags_make_separate_runs() {
    let lines = lines_of("<p>Hello <b>world</b>!</p>");

    let runs = &lines[0].runs;
    assert_eq!(runs.len(), 3);
    assert_eq!(runs[0].style.font_weight, 400);
    assert_eq!(runs[1].style.font_weight, 700);
    assert_eq!(runs[2].style.font_weight, 400);
}

#[test]
fn block_tags_break_lines() {
    assert_eq!(texts_of("<div>one</div><div>two</div>"), vec!["one", "two"]);
}

#[test]
fn the_body_margin_gives_the_first_gap() {
    assert_eq!(gaps_of("<div>one</div><div>two</div>"), vec![8.0, 0.0]);
}

#[test]
fn paragraph_margins_collapse_between_paragraphs() {
    assert_eq!(gaps_of("<p>a</p><p>b</p>"), vec![16.0, 16.0]);
}

#[test]
fn the_larger_of_two_touching_margins_wins() {
    let html = "<div style='margin-bottom: 30px'>a</div><div style='margin-top: 10px'>b</div>";

    assert_eq!(gaps_of(html), vec![8.0, 30.0]);
}

#[test]
fn whitespace_is_collapsed() {
    assert_eq!(texts_of("<p>a   \n   b</p>"), vec!["a b"]);
}

#[test]
fn scripts_and_styles_are_hidden() {
    assert_eq!(
        texts_of("<p>a</p><script>x</script><style>y</style>"),
        vec!["a"]
    );
}

#[test]
fn display_none_hides_a_whole_subtree() {
    assert_eq!(
        texts_of(
            "<style>.gone { display: none }</style><p>a</p><div class=gone><p>b</p></div><p>c</p>"
        ),
        vec!["a", "c"]
    );
}

#[test]
fn list_items_get_bullets_and_indent() {
    let lines = lines_of("<ul><li>One</li><li>Two</li></ul>");

    let texts: Vec<String> = lines.iter().map(Line::plain_text).collect();
    assert_eq!(texts, vec!["\u{2022} One", "\u{2022} Two"]);
    assert_eq!(lines[0].indent, 48.0);
    assert_eq!(lines[1].indent, 48.0);
}

#[test]
fn line_breaks_split_lines() {
    assert_eq!(texts_of("<p>a<br>b</p>"), vec!["a", "b"]);
}

#[test]
fn an_empty_document_has_no_lines() {
    assert!(lines_of("").is_empty());
}

#[test]
fn a_non_breaking_space_is_kept() {
    assert_eq!(texts_of("<p>a&nbsp;b</p>"), vec!["a\u{a0}b"]);
}

#[test]
fn headings_use_their_font_size_and_weight() {
    let lines = lines_of("<h1>Big</h1>");

    assert_eq!(lines[0].runs[0].style.font_size, 32.0);
    assert_eq!(lines[0].runs[0].style.font_weight, 700);
}

#[test]
fn author_colors_reach_the_text() {
    let lines = lines_of("<style>p { color: #ff0000 }</style><p>red</p>");

    assert_eq!(lines[0].runs[0].style.color, Rgba::opaque(255, 0, 0));
}

#[test]
fn a_block_background_becomes_a_backdrop_at_the_block_edge() {
    let lines = lines_of("<div style='background-color: #ffcc00; padding-left: 10px'>x</div>");

    assert_eq!(
        lines[0].background,
        Some(Backdrop {
            color: Rgba::opaque(255, 204, 0),
            left: 8.0,
        })
    );
    assert_eq!(lines[0].indent, 18.0);
}

#[test]
fn a_nested_block_inherits_the_backdrop_of_its_parent() {
    let lines = lines_of("<div style='background-color: #ffcc00'><p>x</p></div>");

    assert_eq!(
        lines[0].background.map(|backdrop| backdrop.color),
        Some(Rgba::opaque(255, 204, 0))
    );
}

#[test]
fn the_body_background_goes_to_the_canvas_not_to_lines() {
    let document = parse_html("<style>body { background-color: #eeeeee }</style><p>x</p>");
    let base = Url::parse("file:///page.html").expect("url");
    let author = collect_author_sheets(&document, &base);
    let styles = compute_styles(&document, &user_agent_sheet(), &author);

    assert_eq!(
        canvas_background(&document, &styles),
        Rgba::opaque(238, 238, 238)
    );
    assert_eq!(page_lines(&document, &styles)[0].background, None);
}

#[test]
fn the_canvas_is_white_by_default() {
    let document = parse_html("<p>x</p>");
    let styles = compute_styles(&document, &user_agent_sheet(), &[]);

    assert_eq!(canvas_background(&document, &styles), Rgba::WHITE);
}

#[test]
fn the_title_is_trimmed_and_collapsed() {
    let document = parse_html("<title>  Hi   there </title><p>x</p>");

    assert_eq!(document_title(&document), Some("Hi there".to_owned()));
}

#[test]
fn a_missing_title_is_none() {
    assert_eq!(document_title(&parse_html("<p>x</p>")), None);
}
