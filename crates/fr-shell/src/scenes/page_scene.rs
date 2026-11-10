use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_geom::color::rgba::Rgba;
use fr_image::store::image_store::ImageStore;
use fr_layout::page::layout_page::layout_page;
use fr_page::load::page::Page;
use fr_paint::build::build_display_list::build_display_list;
use fr_paint::paint::paint_display_list::paint_display_list;
use fr_raster::draw::clear::clear;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_ui::scroll::scroll_metrics::ScrollMetrics;
use fr_ui::scroll::scroll_state::ScrollState;
use fr_ui::scroll::scrollbar_thumb::scrollbar_thumb;
use fr_window::input::input_event::InputEvent;
use fr_window::scene::render_scene::RenderScene;
use fr_window::scene::scene_reaction::SceneReaction;

use crate::scenes::cached_layout::CachedLayout;

pub struct PageScene {
    document: Document,
    styles: StyleMap,
    images: ImageStore,
    text_engine: TextEngine,
    cached: Option<CachedLayout>,
    scroll: ScrollState,
    viewport_height: f32,
}

impl PageScene {
    pub fn new(page: Page) -> Self {
        Self {
            document: page.document,
            styles: page.styles,
            images: page.images,
            text_engine: TextEngine::new(),
            cached: None,
            scroll: ScrollState::new(),
            viewport_height: 0.0,
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
            height: tree.height,
            background: tree.background,
            list: build_display_list(&tree.root),
        });
    }
}

impl PageScene {
    fn metrics(&self) -> Option<ScrollMetrics> {
        let cached = self.cached.as_ref()?;
        Some(ScrollMetrics::new(cached.height, self.viewport_height))
    }
}

impl RenderScene for PageScene {
    fn render(&mut self, canvas: &mut Canvas) {
        self.viewport_height = canvas.height() as f32;
        self.layout_for(canvas.width());
        let Some(metrics) = self.metrics() else {
            return;
        };
        self.scroll.clamp(metrics);
        let Some(cached) = &self.cached else {
            return;
        };
        clear(canvas, cached.background);
        paint_display_list(
            canvas,
            &mut self.text_engine,
            &cached.list,
            self.scroll.offset(),
        );
        if let Some(thumb) = scrollbar_thumb(self.scroll.offset(), metrics, canvas.width() as f32) {
            fill_rect(canvas, thumb, Rgba::new(0, 0, 0, 90));
        }
    }

    fn handle(&mut self, event: InputEvent) -> SceneReaction {
        let Some(metrics) = self.metrics() else {
            return SceneReaction::Idle;
        };
        let changed = match event {
            InputEvent::Wheel { delta_y } => self.scroll.scroll_by(delta_y, metrics),
            InputEvent::Key(key) => self.scroll.apply_key(key, metrics),
            InputEvent::Click { .. } => false,
        };
        if changed {
            SceneReaction::Redraw
        } else {
            SceneReaction::Idle
        }
    }
}
