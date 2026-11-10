mod support;

use std::sync::Arc;

use fr_dom::tree::document::Document;
use fr_html::parse::parse_html::parse_html;
use fr_image::decode::decoded_image::DecodedImage;
use fr_image::store::image_store::ImageStore;
use fr_layout::boxes::box_kind::BoxKind;
use support::{Laid, find_by_id, layout_document};

fn picture(width: u32, height: u32) -> Arc<DecodedImage> {
    Arc::new(DecodedImage::new(
        width,
        height,
        vec![255; (width * height * 4) as usize],
    ))
}

fn with_image(html: &str, css: &str, width: f32, size: (u32, u32)) -> Laid {
    let document: Document = parse_html(html);
    let mut images = ImageStore::new();
    images.insert(find_by_id(&document, "img"), picture(size.0, size.1));
    layout_document(document, css, &images, width)
}

#[test]
fn an_image_keeps_its_natural_size() {
    let page = with_image("<p><img id=img src=x.png></p>", "", 800.0, (120, 60));

    let rect = page.boxed("img").border_box;
    assert_eq!((rect.size.width, rect.size.height), (120.0, 60.0));
    assert!(matches!(page.boxed("img").kind, BoxKind::Image(_)));
}

#[test]
fn a_width_attribute_scales_with_the_aspect_ratio() {
    let page = with_image(
        "<p><img id=img src=x.png width=60></p>",
        "",
        800.0,
        (120, 60),
    );

    let rect = page.boxed("img").border_box;
    assert_eq!((rect.size.width, rect.size.height), (60.0, 30.0));
}

#[test]
fn both_attributes_set_the_exact_size() {
    let page = with_image(
        "<p><img id=img src=x.png width=30 height=30></p>",
        "",
        800.0,
        (120, 60),
    );

    let rect = page.boxed("img").border_box;
    assert_eq!((rect.size.width, rect.size.height), (30.0, 30.0));
}

#[test]
fn css_size_beats_the_attributes() {
    let page = with_image(
        "<p><img id=img src=x.png width=30 style='width: 90px'></p>",
        "",
        800.0,
        (120, 60),
    );

    assert_eq!(page.boxed("img").border_box.size.width, 90.0);
}

#[test]
fn a_wide_image_shrinks_to_the_available_width() {
    let page = with_image("<p><img id=img src=x.png></p>", "", 316.0, (600, 300));

    let rect = page.boxed("img").border_box;
    assert_eq!(rect.size.width, 300.0);
    assert_eq!(rect.size.height, 150.0);
}

#[test]
fn max_width_limits_the_image() {
    let page = with_image(
        "<p><img id=img src=x.png style='max-width: 50%'></p>",
        "",
        816.0,
        (600, 300),
    );

    assert_eq!(page.boxed("img").border_box.size.width, 400.0);
}

#[test]
fn text_align_center_centers_an_inline_image() {
    let page = with_image(
        "<div style='text-align: center'><img id=img src=x.png></div>",
        "",
        816.0,
        (100, 50),
    );

    assert_eq!(
        page.boxed("img").border_box.origin.x,
        8.0 + (800.0 - 100.0) / 2.0
    );
}

#[test]
fn a_block_image_with_auto_margins_is_centered() {
    let page = with_image(
        "<img id=img src=x.png style='display: block; margin: 0 auto'>",
        "",
        816.0,
        (100, 50),
    );

    assert_eq!(
        page.boxed("img").border_box.origin.x,
        8.0 + (800.0 - 100.0) / 2.0
    );
}

#[test]
fn an_image_pushes_following_text_below_it() {
    let page = with_image(
        "<div id=d>before<img id=img src=x.png>after</div>",
        "",
        800.0,
        (50, 40),
    );

    let image = page.boxed("img").border_box;
    let container = page.boxed("d");
    let texts: Vec<_> = container
        .children
        .iter()
        .filter(|child| matches!(child.kind, BoxKind::Text(_)))
        .collect();
    assert_eq!(texts.len(), 2);
    assert!(texts[0].border_box.bottom() <= image.origin.y);
    assert!(texts[1].border_box.origin.y >= image.bottom());
}

#[test]
fn a_missing_image_shows_its_alt_text() {
    let page = layout_document(
        parse_html("<p id=p>see <img src=gone.png alt='[gone]'> here</p>"),
        "",
        &ImageStore::new(),
        800.0,
    );

    let paragraph = page.boxed("p");
    let BoxKind::Text(text) = &paragraph.children[0].kind else {
        panic!("text box");
    };
    assert_eq!(text.plain_text(), "see [gone] here");
}

#[test]
fn a_missing_image_without_alt_leaves_no_trace() {
    let page = layout_document(
        parse_html("<p id=p>see <img src=gone.png> here</p>"),
        "",
        &ImageStore::new(),
        800.0,
    );

    let BoxKind::Text(text) = &page.boxed("p").children[0].kind else {
        panic!("text box");
    };
    assert_eq!(text.plain_text(), "see here");
}
