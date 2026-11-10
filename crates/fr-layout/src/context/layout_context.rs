use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_image::store::image_store::ImageStore;
use fr_text::engine::text_engine::TextEngine;

pub struct LayoutContext<'a> {
    pub document: &'a Document,
    pub styles: &'a StyleMap,
    pub images: &'a ImageStore,
    pub text: &'a mut TextEngine,
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
        }
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
