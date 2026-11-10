use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_history::history::navigation_history::NavigationHistory;
use fr_image::store::image_store::ImageStore;
use fr_layout::page::layout_page::layout_page;
use fr_page::load::page::Page;
use fr_paint::build::build_display_list::build_display_list;
use fr_paint::paint::paint_display_list::paint_display_list;
use fr_raster::draw::clear::clear;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_ui::address::address_field::AddressField;
use fr_ui::address::address_outcome::AddressOutcome;
use fr_ui::address::apply_address_key::apply_address_key;
use fr_ui::scroll::scroll_metrics::ScrollMetrics;
use fr_ui::scroll::scroll_state::ScrollState;
use fr_ui::scroll::scrollbar_thumb::scrollbar_thumb;
use fr_ui::toolbar::hit_toolbar::hit_toolbar;
use fr_ui::toolbar::toolbar_metrics::TOOLBAR_HEIGHT;
use fr_ui::toolbar::toolbar_target::ToolbarTarget;
use fr_window::input::input_event::InputEvent;
use fr_window::input::key_input::KeyInput;
use fr_window::scene::pointer_kind::PointerKind;
use fr_window::scene::render_scene::RenderScene;
use fr_window::scene::scene_reaction::SceneReaction;
use url::Url;

use crate::browser::window_title::window_title;
use crate::scenes::cached_layout::CachedLayout;
use crate::scenes::fetch_page::fetch_page;
use crate::scenes::link_target::link_target;
use crate::toolbar::draw_toolbar::draw_toolbar;
use crate::toolbar::toolbar_state::ToolbarState;

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
    viewport_width: f32,
    viewport_height: f32,
    history: NavigationHistory,
    address: AddressField,
    is_over_link: bool,
}

impl PageScene {
    pub fn new(page: Page) -> Self {
        let mut history = NavigationHistory::new();
        history.visit(page.url.clone());
        let mut address = AddressField::new();
        address.set_text(page.url.as_str());
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
            viewport_width: 0.0,
            viewport_height: 0.0,
            history,
            address,
            is_over_link: false,
        }
    }

    fn show(&mut self, page: Page, scroll: f32) {
        self.title = window_title(&page);
        self.address.set_text(page.url.as_str());
        self.address.blur();
        self.url = page.url;
        self.document = page.document;
        self.styles = page.styles;
        self.images = page.images;
        self.cached = None;
        self.scroll = ScrollState::new();
        self.pending_scroll = Some(scroll);
        self.is_over_link = false;
    }

    fn open(&mut self, input: &str) -> SceneReaction {
        let Some(page) = fetch_page(input) else {
            return SceneReaction::Redraw;
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
        let Some(page) = fetch_page(url.as_str()) else {
            return SceneReaction::Idle;
        };
        self.show(page, scroll);
        SceneReaction::Redraw
    }

    fn reload(&mut self) -> SceneReaction {
        let Some(page) = fetch_page(self.url.as_str()) else {
            return SceneReaction::Idle;
        };
        let scroll = self.scroll.offset();
        self.show(page, scroll);
        SceneReaction::Redraw
    }

    fn target_at(&mut self, x: f32, y: f32) -> Option<Url> {
        if y < TOOLBAR_HEIGHT {
            return None;
        }
        let cached = self.cached.as_ref()?;
        let point = Point::new(x, y - TOOLBAR_HEIGHT + self.scroll.offset());
        link_target(
            cached,
            &self.document,
            &mut self.text_engine,
            &self.url,
            point,
        )
    }

    fn click(&mut self, x: f32, y: f32) -> SceneReaction {
        if let Some(target) = hit_toolbar(Point::new(x, y), self.viewport_width) {
            return match target {
                ToolbarTarget::Back => self.travel(false),
                ToolbarTarget::Forward => self.travel(true),
                ToolbarTarget::Reload => self.reload(),
                ToolbarTarget::Address => {
                    self.address.focus();
                    SceneReaction::Redraw
                }
                ToolbarTarget::Empty => SceneReaction::Idle,
            };
        }
        let was_focused = self.address.is_focused();
        self.address.blur();
        match self.target_at(x, y) {
            Some(url) => self.open(url.as_str()),
            None if was_focused => SceneReaction::Redraw,
            None => SceneReaction::Idle,
        }
    }

    fn press(&mut self, key: KeyInput) -> SceneReaction {
        if self.address.is_focused() {
            match apply_address_key(&mut self.address, key) {
                AddressOutcome::Edited => return SceneReaction::Redraw,
                AddressOutcome::Submit => {
                    let input = self.address.text().to_owned();
                    self.address.blur();
                    return self.open(&input);
                }
                AddressOutcome::Cancel => {
                    self.address.set_text(self.url.as_str());
                    self.address.blur();
                    return SceneReaction::Redraw;
                }
                AddressOutcome::Ignored => {}
            }
        }
        match key {
            KeyInput::GoBack => self.travel(false),
            KeyInput::GoForward => self.travel(true),
            KeyInput::Reload => self.reload(),
            KeyInput::FocusAddress => {
                self.address.focus();
                SceneReaction::Redraw
            }
            _ => self.scroll_for(InputEvent::Key(key)),
        }
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
        self.viewport_width = canvas.width() as f32;
        self.viewport_height = (canvas.height() as f32 - TOOLBAR_HEIGHT).max(0.0);
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
            self.scroll.offset() - TOOLBAR_HEIGHT,
        );
        if let Some(thumb) = scrollbar_thumb(self.scroll.offset(), metrics, self.viewport_width) {
            let placed = Rect::from_xywh(
                thumb.origin.x,
                thumb.origin.y + TOOLBAR_HEIGHT,
                thumb.size.width,
                thumb.size.height,
            );
            fill_rect(canvas, placed, Rgba::new(0, 0, 0, 90));
        }
        let state = ToolbarState {
            field: &self.address,
            can_go_back: self.history.can_go_back(),
            can_go_forward: self.history.can_go_forward(),
        };
        draw_toolbar(canvas, &mut self.text_engine, &state);
    }

    fn handle(&mut self, event: InputEvent) -> SceneReaction {
        match event {
            InputEvent::Click { x, y } => self.click(x, y),
            InputEvent::PointerMoved { x, y } => {
                self.is_over_link = self.target_at(x, y).is_some();
                SceneReaction::Idle
            }
            InputEvent::Character(character) if self.address.is_focused() => {
                self.address.insert(character);
                SceneReaction::Redraw
            }
            InputEvent::Character(_) => SceneReaction::Idle,
            InputEvent::Key(key) => self.press(key),
            InputEvent::Wheel { .. } => self.scroll_for(event),
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
