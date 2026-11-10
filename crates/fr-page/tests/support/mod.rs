#![allow(dead_code)]

use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_layout::boxes::box_kind::BoxKind;
use fr_layout::boxes::layout_box::LayoutBox;
use fr_layout::page::layout_page::layout_page;
use fr_layout::page::layout_tree::LayoutTree;
use fr_page::load::page::Page;
use fr_text::engine::text_engine::TextEngine;

pub fn layout_of(page: &Page, width: f32) -> LayoutTree {
    let mut engine = TextEngine::new();
    layout_page(
        &page.document.borrow(),
        &page.styles,
        &page.images,
        &mut engine,
        width,
    )
}

pub fn texts(tree: &LayoutTree) -> Vec<String> {
    let mut found = Vec::new();
    let mut pending: Vec<&LayoutBox> = vec![&tree.root];
    while let Some(current) = pending.pop() {
        if let BoxKind::Text(text) = &current.kind {
            found.push(text.plain_text());
        }
        pending.extend(current.children.iter().rev());
    }
    found
}

pub fn box_of_first<'a>(
    tree: &'a LayoutTree,
    document: &Document,
    tag: &str,
) -> Option<&'a LayoutBox> {
    let id = document.walk(document.root()).find_map(|edge| match edge {
        Edge::Enter(id) => {
            let element = document.node(id)?.kind().as_element()?;
            (element.name() == tag).then_some(id)
        }
        Edge::Leave(_) => None,
    })?;
    let mut pending: Vec<&LayoutBox> = vec![&tree.root];
    while let Some(current) = pending.pop() {
        if current.node == Some(id) {
            return Some(current);
        }
        pending.extend(current.children.iter());
    }
    None
}
