use fr_layout::page::layout_page::layout_page;
use fr_page::load::build_page::build_page;
use fr_paint::build::build_display_list::build_display_list;
use fr_tab::footprint::estimate_footprint::estimate_footprint;
use fr_tab::model::layout_cache::LayoutCache;
use fr_tab::model::tab_content::TabContent;
use fr_text::engine::text_engine::TextEngine;
use url::Url;

fn content_of(html: &str) -> TabContent {
    let url = Url::parse("https://example.com/").unwrap();
    TabContent::from_page(build_page(html, url))
}

fn with_layout(mut content: TabContent) -> TabContent {
    let mut engine = TextEngine::new();
    let tree = layout_page(
        &content.document.borrow(),
        &content.styles,
        &content.images,
        &mut engine,
        800.0,
    );
    content.layout = Some(LayoutCache {
        width: 800,
        height: tree.height,
        background: tree.background,
        list: build_display_list(&tree.root),
        root: tree.root,
    });
    content
}

#[test]
fn a_bigger_document_has_a_bigger_footprint() {
    let small = content_of("<p>one</p>");
    let big = content_of(&"<p>paragraph of text</p>".repeat(200));

    assert!(estimate_footprint(&big) > estimate_footprint(&small));
}

#[test]
fn layout_adds_to_the_footprint() {
    let plain = content_of("<p>hello</p><p>world</p>");
    let laid_out = with_layout(content_of("<p>hello</p><p>world</p>"));

    assert!(estimate_footprint(&laid_out) > estimate_footprint(&plain));
}

#[test]
fn an_empty_document_is_tiny() {
    let content = content_of("");

    assert!(estimate_footprint(&content) < 10_000);
}
