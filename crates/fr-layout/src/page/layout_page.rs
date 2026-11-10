use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_geom::geometry::rect::Rect;
use fr_image::store::image_store::ImageStore;
use fr_text::engine::text_engine::TextEngine;

use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::flow::layout_flow::layout_flow;
use crate::metrics::content_area::ContentArea;
use crate::page::canvas_background::canvas_background;
use crate::page::layout_tree::LayoutTree;
use crate::page::run_with_large_stack::run_with_large_stack;

pub fn layout_page(
    document: &Document,
    styles: &StyleMap,
    images: &ImageStore,
    text: &mut TextEngine,
    viewport_width: f32,
) -> LayoutTree {
    let width = viewport_width.max(1.0);
    run_with_large_stack(|| {
        let mut context = LayoutContext::new(document, styles, images, &mut *text);
        let flow = layout_flow(
            &mut context,
            document.root(),
            &ComputedStyle::initial(),
            ContentArea {
                x: 0.0,
                y: 0.0,
                width,
            },
            false,
            false,
        );
        let height = flow.content_height;
        let mut root = LayoutBox::plain(
            None,
            BoxKind::Block,
            Rect::from_xywh(0.0, 0.0, width, height),
        );
        root.children = flow.boxes;
        LayoutTree {
            root,
            height,
            background: canvas_background(document, styles),
        }
    })
}
