use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_image::store::image_store::ImageStore;
use fr_layout::page::layout_page::layout_page;
use fr_page::load::page::Page;
use fr_paint::build::build_display_list::build_display_list;
use fr_paint::paint::paint_display_list::paint_display_list;
use fr_raster::draw::clear::clear;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_window::scene::render_scene::RenderScene;

use crate::scenes::cached_layout::CachedLayout;

pub struct PageScene {
    document: Document,
    styles: StyleMap,
    images: ImageStore,
    text_engine: TextEngine,
    cached: Option<CachedLayout>,
}

impl PageScene {
    pub fn new(page: Page) -> Self {
        Self {
            document: page.document,
            styles: page.styles,
            images: page.images,
            text_engine: TextEngine::new(),
            cached: None,
        }
    }

    fn layout_for(&mut self, width: u32) {
        let is_current = self
            .cached
            .as_ref()
            .is_some_and(|cached| cached.width == width);
        if is_current {
            return;
        }
        let tree = layout_page(
            &self.document,
            &self.styles,
            &self.images,
            &mut self.text_engine,
            width as f32,
        );
        self.cached = Some(CachedLayout {
            width,
            background: tree.background,
            list: build_display_list(&tree.root),
        });
    }
}

impl RenderScene for PageScene {
    fn render(&mut self, canvas: &mut Canvas) {
        self.layout_for(canvas.width());
        let Some(cached) = &self.cached else {
            return;
        };
        clear(canvas, cached.background);
        paint_display_list(canvas, &mut self.text_engine, &cached.list, 0.0);
    }
}
