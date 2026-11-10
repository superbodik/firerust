use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::cascade::style_map::StyleMap;
use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_geom::color::rgba::Rgba;

use crate::lines::context_stack::ContextStack;
use crate::lines::line::Line;
use crate::lines::line_builder::LineBuilder;
use crate::lines::run_style::RunStyle;

pub fn page_lines(document: &Document, styles: &StyleMap) -> Vec<Line> {
    let mut builder = LineBuilder::new();
    let mut contexts = ContextStack::new();
    let mut hidden_root: Option<NodeId> = None;

    for edge in document.walk(document.root()) {
        if let Some(root) = hidden_root {
            if edge == Edge::Leave(root) {
                hidden_root = None;
            }
            continue;
        }
        match edge {
            Edge::Enter(id) => {
                if enter(document, styles, id, &mut builder, &mut contexts) {
                    hidden_root = Some(id);
                }
            }
            Edge::Leave(id) => leave(styles, id, &mut builder, &mut contexts),
        }
    }

    builder.finish()
}

fn enter(
    document: &Document,
    styles: &StyleMap,
    id: NodeId,
    builder: &mut LineBuilder,
    contexts: &mut ContextStack,
) -> bool {
    let Some(node) = document.node(id) else {
        return false;
    };
    if let Some(text) = node.kind().as_text() {
        let style = node
            .parent()
            .and_then(|parent| styles.get(parent))
            .map_or_else(
                || RunStyle::from_computed(&ComputedStyle::initial()),
                RunStyle::from_computed,
            );
        builder.push_text(text, style, contexts.current());
        return false;
    }
    let Some(style) = styles.get(id) else {
        return false;
    };
    if element_name(document, id) == Some("br") {
        builder.break_line();
    }
    match style.display {
        Display::None => return true,
        Display::Inline => {}
        Display::Block => open_block(document, id, style, builder, contexts),
        Display::ListItem => {
            open_block(document, id, style, builder, contexts);
            builder.start_bullet(RunStyle::from_computed(style), contexts.current());
        }
    }
    false
}

fn open_block(
    document: &Document,
    id: NodeId,
    style: &ComputedStyle,
    builder: &mut LineBuilder,
    contexts: &mut ContextStack,
) {
    builder.break_line();
    builder.add_gap(style.margin.top.px_or_zero());
    let inset = style.padding.left.px_or_zero() + style.border_width.left;
    contexts.push(
        style.margin.left.px_or_zero(),
        inset,
        background_for(document, id, style),
    );
}

fn background_for(document: &Document, id: NodeId, style: &ComputedStyle) -> Option<Rgba> {
    let is_root_element = matches!(element_name(document, id), Some("html" | "body"));
    (!is_root_element && style.background_color.a > 0).then_some(style.background_color)
}

fn leave(styles: &StyleMap, id: NodeId, builder: &mut LineBuilder, contexts: &mut ContextStack) {
    let Some(style) = styles.get(id) else {
        return;
    };
    if matches!(style.display, Display::Block | Display::ListItem) {
        builder.break_line();
        builder.add_gap(style.margin.bottom.px_or_zero());
        contexts.pop();
    }
}
