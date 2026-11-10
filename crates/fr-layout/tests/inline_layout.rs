mod support;

use fr_layout::boxes::box_kind::BoxKind;
use fr_layout::boxes::layout_box::LayoutBox;
use fr_text::render::text_alignment::TextAlignment;
use support::layout;

const LONG_TEXT: &str = "<p id=a>The quick brown fox jumps over the lazy dog again and again and again until the line has to wrap.</p>";

fn text_boxes(root: &LayoutBox) -> Vec<&LayoutBox> {
    let mut found = Vec::new();
    let mut pending = vec![root];
    while let Some(current) = pending.pop() {
        if matches!(current.kind, BoxKind::Text(_)) {
            found.push(current);
        }
        pending.extend(current.children.iter().rev());
    }
    found
}

fn text_of(layout_box: &LayoutBox) -> String {
    match &layout_box.kind {
        BoxKind::Text(text) => text.plain_text(),
        _ => String::new(),
    }
}

#[test]
fn text_wraps_in_a_narrow_viewport() {
    let wide = layout(LONG_TEXT, "", 1200.0);
    let narrow = layout(LONG_TEXT, "", 160.0);

    assert!(
        narrow.boxed("a").border_box.size.height > wide.boxed("a").border_box.size.height * 2.0
    );
}

#[test]
fn wider_text_needs_more_height_than_one_line() {
    let page = layout("<p id=a>short</p>", "", 800.0);

    let height = page.boxed("a").border_box.size.height;
    assert!(height > 10.0 && height < 40.0);
}

#[test]
fn a_block_between_text_splits_the_inline_content() {
    let page = layout("<div id=a>one<div id=b>two</div>three</div>", "", 800.0);

    let kinds: Vec<&str> = page
        .boxed("a")
        .children
        .iter()
        .map(|child| match child.kind {
            BoxKind::Text(_) => "text",
            BoxKind::Block => "block",
            _ => "other",
        })
        .collect();
    assert_eq!(kinds, vec!["text", "block", "text"]);
}

#[test]
fn whitespace_collapses_into_single_spaces() {
    let page = layout("<p id=a>  a   \n   b  </p>", "", 800.0);

    let texts = text_boxes(page.boxed("a"));
    assert_eq!(text_of(texts[0]), "a b");
}

#[test]
fn a_line_break_starts_a_new_line() {
    let single = layout("<p id=a>a b</p>", "", 800.0);
    let broken = layout("<p id=a>a<br>b</p>", "", 800.0);

    assert!(
        broken.boxed("a").border_box.size.height > single.boxed("a").border_box.size.height * 1.5
    );
}

#[test]
fn preformatted_text_keeps_its_spaces_and_lines() {
    let page = layout("<pre id=a>a   b\n  c</pre>", "", 800.0);

    let texts = text_boxes(page.boxed("a"));
    assert_eq!(text_of(texts[0]), "a   b\n  c");
}

#[test]
fn inline_elements_make_separate_runs_in_one_box() {
    let page = layout("<p id=a>x <b>y</b> <i>z</i></p>", "", 800.0);

    let texts = text_boxes(page.boxed("a"));
    assert_eq!(texts.len(), 1);
    let BoxKind::Text(text) = &texts[0].kind else {
        panic!("text box");
    };
    assert_eq!(text.plain_text(), "x y z");
    assert_eq!(text.runs.len(), 3);
    assert_eq!(text.runs[1].font_weight, 700);
    assert!(text.runs[2].is_italic);
}

#[test]
fn headings_use_a_bigger_font() {
    let page = layout("<h1 id=a>Big</h1>", "", 800.0);

    let texts = text_boxes(page.boxed("a"));
    let BoxKind::Text(text) = &texts[0].kind else {
        panic!("text box");
    };
    assert_eq!(text.runs[0].font_size, 32.0);
    assert_eq!(text.runs[0].font_weight, 700);
}

#[test]
fn text_align_reaches_the_text_box() {
    let page = layout("<p id=a style='text-align: center'>x</p>", "", 800.0);

    let texts = text_boxes(page.boxed("a"));
    let BoxKind::Text(text) = &texts[0].kind else {
        panic!("text box");
    };
    assert_eq!(text.alignment, TextAlignment::Center);
}

#[test]
fn line_height_scales_the_text_height() {
    let normal = layout("<p id=a>x</p>", "", 800.0);
    let tall = layout("<p id=a style='line-height: 3'>x</p>", "", 800.0);

    assert!(
        tall.boxed("a").border_box.size.height > normal.boxed("a").border_box.size.height * 1.8
    );
}

#[test]
fn a_text_box_spans_the_content_width() {
    let page = layout("<div id=a style='width: 300px'>hello</div>", "", 800.0);

    let texts = text_boxes(page.boxed("a"));
    assert_eq!(texts[0].border_box.size.width, 300.0);
}

#[test]
fn hidden_text_makes_no_text_box() {
    let page = layout(
        "<div id=a><script>var x = 1;</script><style>p{}</style></div>",
        "",
        800.0,
    );

    assert!(text_boxes(page.boxed("a")).is_empty());
}
