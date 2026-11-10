use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_history::history::navigation_history::NavigationHistory;
use fr_image::store::image_store::ImageStore;
use fr_layout::page::layout_page::layout_page;
use fr_page::load::load_page::load_page;
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
use fr_window::input::key_input::KeyInput;
use fr_window::scene::pointer_kind::PointerKind;
use fr_window::scene::render_scene::RenderScene;
use fr_window::scene::scene_reaction::SceneReaction;
use url::Url;

use crate::browser::window_title::window_title;
use crate::scenes::cached_layout::CachedLayout;
use crate::scenes::link_target::link_target;

pub struct PageScene {
    url: Url,
    title: String,
    document: Document,
    styles: StyleMap,
    images: ImageStore,
    text_engine: TextEngine,
    cached: Option<CachedLayout>,
    scroll: ScrollState,
    pending_scroll: Option<f32>,
    viewport_height: f32,
    history: NavigationHistory,
    is_over_link: bool,
}

impl PageScene {
    pub fn new(page: Page) -> Self {
        let mut history = NavigationHistory::new();
        history.visit(page.url.clone());
        Self {
            title: window_title(&page),
            url: page.url,
            document: page.document,
            styles: page.styles,
            images: page.images,
            text_engine: TextEngine::new(),
            cached: None,
            scroll: ScrollState::new(),
            pending_scroll: None,
            viewport_height: 0.0,
            history,
            is_over_link: false,
        }
    }

    fn show(&mut self, page: Page, scroll: f32) {
        self.title = window_title(&page);
        self.url = page.url;
        self.document = page.document;
        self.styles = page.styles;
        self.images = page.images;
        self.cached = None;
        self.scroll = ScrollState::new();
        self.pending_scroll = Some(scroll);
        self.is_over_link = false;
    }

    fn fetch(&self, url: &Url) -> Option<Page> {
        let base_dir = std::env::current_dir().unwrap_or_default();
        match load_page(url.as_str(), &base_dir) {
            Ok(page) => Some(page),
            Err(error) => {
                eprintln!("cannot open {url}: {error}");
                None
            }
        }
    }

    fn open(&mut self, url: &Url) -> SceneReaction {
        let Some(page) = self.fetch(url) else {
            return SceneReaction::Idle;
        };
        self.history.remember_scroll(self.scroll.offset());
        self.history.visit(page.url.clone());
        self.show(page, 0.0);
        SceneReaction::Redraw
    }

    fn travel(&mut self, forward: bool) -> SceneReaction {
        self.history.remember_scroll(self.scroll.offset());
        let entry = if forward {
            self.history.go_forward()
        } else {
            self.history.go_back()
        };
        let Some((url, scroll)) = entry.map(|entry| (entry.url.clone(), entry.scroll)) else {
            return SceneReaction::Idle;
        };
        let Some(page) = self.fetch(&url) else {
            return SceneReaction::Idle;
        };
        self.show(page, scroll);
        SceneReaction::Redraw
    }

    fn reload(&mut self) -> SceneReaction {
        let url = self.url.clone();
        let Some(page) = self.fetch(&url) else {
            return SceneReaction::Idle;
        };
        let scroll = self.scroll.offset();
        self.show(page, scroll);
        SceneReaction::Redraw
    }

    fn target_at(&mut self, x: f32, y: f32) -> Option<Url> {
        let cached = self.cached.as_ref()?;
        let point = Point::new(x, y + self.scroll.offset());
        link_target(
            cached,
            &self.document,
            &mut self.text_engine,
            &self.url,
            point,
        )
    }

    fn scroll_for(&mut self, event: InputEvent) -> SceneReaction {
        let Some(metrics) = self.metrics() else {
            return SceneReaction::Idle;
        };
        let changed = match event {
            InputEvent::Wheel { delta_y } => self.scroll.scroll_by(delta_y, metrics),
            InputEvent::Key(key) => self.scroll.apply_key(key, metrics),
            _ => false,
        };
        if changed {
            SceneReaction::Redraw
        } else {
            SceneReaction::Idle
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
            root: tree.root,
        });
    }

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
        if let Some(scroll) = self.pending_scroll.take() {
            self.scroll.scroll_to(scroll, metrics);
        }
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
        match event {
            InputEvent::Click { x, y } => match self.target_at(x, y) {
                Some(url) => self.open(&url),
                None => SceneReaction::Idle,
            },
            InputEvent::PointerMoved { x, y } => {
                self.is_over_link = self.target_at(x, y).is_some();
                SceneReaction::Idle
            }
            InputEvent::Key(KeyInput::GoBack) => self.travel(false),
            InputEvent::Key(KeyInput::GoForward) => self.travel(true),
            InputEvent::Key(KeyInput::Reload) => self.reload(),
            InputEvent::Wheel { .. } | InputEvent::Key(_) => self.scroll_for(event),
        }
    }

    fn title(&self) -> Option<String> {
        Some(self.title.clone())
    }

    fn pointer(&self) -> PointerKind {
        if self.is_over_link {
            PointerKind::Hand
        } else {
            PointerKind::Arrow
        }
    }
}
