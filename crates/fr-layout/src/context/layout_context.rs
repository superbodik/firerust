use std::collections::HashMap;

use fr_css::cascade::style_map::StyleMap;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_image::store::image_store::ImageStore;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::content_widths::ContentWidths;

use crate::control::control_focus::ControlFocus;

pub struct LayoutContext<'a> {
    pub document: &'a Document,
    pub styles: &'a StyleMap,
    pub images: &'a ImageStore,
    pub text: &'a mut TextEngine,
    pub focus: Option<ControlFocus>,
    pub intrinsic_widths: HashMap<NodeId, ContentWidths>,
}

impl<'a> LayoutContext<'a> {
    pub fn new(
        document: &'a Document,
        styles: &'a StyleMap,
        images: &'a ImageStore,
        text: &'a mut TextEngine,
    ) -> Self {
        Self {
            document,
            styles,
            images,
            text,
            focus: None,
            intrinsic_widths: HashMap::new(),
        }
    }

    pub fn with_focus(mut self, focus: Option<ControlFocus>) -> Self {
        self.focus = focus;
        self
    }

    pub fn document(&self) -> &'a Document {
        self.document
    }

    pub fn styles(&self) -> &'a StyleMap {
        self.styles
    }

    pub fn images(&self) -> &'a ImageStore {
        self.images
    }
}
