#![allow(dead_code)]

use fr_css::cascade::compute_styles::compute_styles;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_html::parse::parse_html::parse_html;
use fr_image::store::image_store::ImageStore;
use fr_layout::boxes::layout_box::LayoutBox;
use fr_layout::page::layout_page::layout_page;
use fr_layout::page::layout_tree::LayoutTree;
use fr_text::engine::text_engine::TextEngine;

pub struct Laid {
    pub document: Document,
    pub tree: LayoutTree,
}

impl Laid {
    pub fn id(&self, wanted: &str) -> NodeId {
        find_by_id(&self.document, wanted)
    }

    pub fn find(&self, wanted: &str) -> Option<&LayoutBox> {
        let id = self.id(wanted);
        find_box(&self.tree.root, id)
    }

    pub fn boxed(&self, wanted: &str) -> &LayoutBox {
        self.find(wanted).expect("box for the element")
    }
}

pub fn find_by_id(document: &Document, wanted: &str) -> NodeId {
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

pub fn find_box(root: &LayoutBox, id: NodeId) -> Option<&LayoutBox> {
    let mut pending: Vec<&LayoutBox> = vec![root];
    while let Some(current) = pending.pop() {
        if current.node == Some(id) {
            return Some(current);
        }
        pending.extend(current.children.iter());
    }
    None
}

pub fn layout(html: &str, css: &str, width: f32) -> Laid {
    let document = parse_html(html);
    layout_document(document, css, &ImageStore::new(), width)
}

pub fn layout_document(document: Document, css: &str, images: &ImageStore, width: f32) -> Laid {
    let author = [parse_stylesheet(css)];
    let styles = compute_styles(&document, &user_agent_sheet(), &author);
    let mut engine = TextEngine::new();
    let tree = layout_page(&document, &styles, images, &mut engine, width);
    Laid { document, tree }
}

pub fn layout_focused(html: &str, css: &str, width: f32, focused: &str, caret: usize) -> Laid {
    use fr_layout::control::control_focus::ControlFocus;
    use fr_layout::page::layout_page::layout_page_with_focus;

    let document = parse_html(html);
    let author = [parse_stylesheet(css)];
    let styles = compute_styles(&document, &user_agent_sheet(), &author);
    let mut engine = TextEngine::new();
    let node = find_by_id(&document, focused);
    let tree = layout_page_with_focus(
        &document,
        &styles,
        &ImageStore::new(),
        &mut engine,
        width,
        Some(ControlFocus { node, caret }),
    );
    Laid { document, tree }
}
