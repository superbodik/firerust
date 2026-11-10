use std::time::Duration;

use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_layout::page::layout_page::layout_page;
use fr_mem::meter::memory_meter::MemoryMeter;
use fr_page::images::shared_image_cache::shared_image_bytes;
use fr_page::load::blank_page::blank_page;
use fr_page::load::page::Page;
use fr_paint::build::build_display_list::build_display_list;
use fr_paint::paint::paint_display_list::paint_display_list;
use fr_raster::draw::clear::clear;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_tab::model::layout_cache::LayoutCache;
use fr_tab::set::tab_set::TabSet;
use fr_tab::summary::memory_summary::memory_summary;
use fr_text::engine::text_engine::TextEngine;
use fr_ui::address::address_field::AddressField;
use fr_ui::address::address_outcome::AddressOutcome;
use fr_ui::address::apply_address_key::apply_address_key;
use fr_ui::scroll::scroll_metrics::ScrollMetrics;
use fr_ui::scroll::scroll_state::ScrollState;
use fr_ui::scroll::scrollbar_thumb::scrollbar_thumb;
use fr_ui::tabs::hit_tab_strip::hit_tab_strip;
use fr_ui::tabs::tab_strip_target::TabStripTarget;
use fr_ui::toolbar::hit_toolbar::hit_toolbar;
use fr_ui::toolbar::toolbar_metrics::CHROME_HEIGHT;
use fr_ui::toolbar::toolbar_target::ToolbarTarget;
use fr_window::input::input_event::InputEvent;
use fr_window::input::key_input::KeyInput;
use fr_window::scene::pointer_kind::PointerKind;
use fr_window::scene::render_scene::RenderScene;
use fr_window::scene::scene_reaction::SceneReaction;
use url::Url;

use crate::scenes::address_text::address_text;
use crate::scenes::fetch_page::fetch_page;
use crate::scenes::link_target::link_target;
use crate::toolbar::draw_memory_bar::draw_memory_bar;
use crate::toolbar::draw_tab_strip::draw_tab_strip;
use crate::toolbar::draw_toolbar::draw_toolbar;
use crate::toolbar::tab_strip_state::TabStripState;
use crate::toolbar::toolbar_state::ToolbarState;

const MAX_LIVE_TABS: usize = 2;
const MEMORY_REFRESH: Duration = Duration::from_secs(1);

pub struct PageScene {
    tabs: TabSet,
    text_engine: TextEngine,
    scroll: ScrollState,
    pending_scroll: Option<f32>,
    viewport_width: f32,
    viewport_height: f32,
    address: AddressField,
    is_over_link: bool,
    meter: MemoryMeter,
    shows_memory: bool,
}

impl PageScene {
    pub fn new(page: Page) -> Self {
        let mut address = AddressField::new();
        address.set_text(&address_text(&page.url));
        Self {
            tabs: TabSet::new(page),
            text_engine: TextEngine::new(),
            scroll: ScrollState::new(),
            pending_scroll: None,
            viewport_width: 0.0,
            viewport_height: 0.0,
            address,
            is_over_link: false,
            meter: MemoryMeter::new(),
            shows_memory: false,
        }
    }

    fn remember_scroll(&mut self) {
        let offset = self.scroll.offset();
        self.tabs.active_mut().set_scroll(offset);
    }

    fn sync_active(&mut self) {
        if self.tabs.active().is_discarded() {
            let address = self.tabs.active().url().to_string();
            if let Some(page) = fetch_page(&address) {
                self.tabs.active_mut().restore(page);
            }
        }
        let scroll = self.tabs.active().scroll();
        self.after_navigation(scroll);
        self.tabs.enforce_limit(MAX_LIVE_TABS);
    }

    fn after_navigation(&mut self, scroll: f32) {
        self.address
            .set_text(&address_text(self.tabs.active().url()));
        self.address.blur();
        self.scroll = ScrollState::new();
        self.pending_scroll = Some(scroll);
        self.is_over_link = false;
    }

    fn switch_to(&mut self, index: usize) -> SceneReaction {
        self.remember_scroll();
        if !self.tabs.activate(index) {
            return SceneReaction::Idle;
        }
        self.sync_active();
        SceneReaction::Redraw
    }

    fn new_tab(&mut self) -> SceneReaction {
        let Some(page) = blank_page() else {
            return SceneReaction::Idle;
        };
        self.remember_scroll();
        self.tabs.open(page);
        self.sync_active();
        self.address.focus();
        SceneReaction::Redraw
    }

    fn close_tab(&mut self, index: usize) -> SceneReaction {
        self.remember_scroll();
        if !self.tabs.close(index) {
            return SceneReaction::Idle;
        }
        self.sync_active();
        SceneReaction::Redraw
    }

    fn open(&mut self, input: &str) -> SceneReaction {
        let Some(page) = fetch_page(input) else {
            return SceneReaction::Redraw;
        };
        let offset = self.scroll.offset();
        let tab = self.tabs.active_mut();
        tab.history_mut().remember_scroll(offset);
        tab.history_mut().visit(page.url.clone());
        tab.show(page);
        self.after_navigation(0.0);
        SceneReaction::Redraw
    }

    fn travel(&mut self, forward: bool) -> SceneReaction {
        let offset = self.scroll.offset();
        let history = self.tabs.active_mut().history_mut();
        history.remember_scroll(offset);
        let entry = if forward {
            history.go_forward()
        } else {
            history.go_back()
        };
        let Some((url, scroll)) = entry.map(|entry| (entry.url.clone(), entry.scroll)) else {
            return SceneReaction::Idle;
        };
        let Some(page) = fetch_page(url.as_str()) else {
            return SceneReaction::Idle;
        };
        self.tabs.active_mut().show(page);
        self.after_navigation(scroll);
        SceneReaction::Redraw
    }

    fn reload(&mut self) -> SceneReaction {
        let address = self.tabs.active().url().to_string();
        let Some(page) = fetch_page(&address) else {
            return SceneReaction::Idle;
        };
        let offset = self.scroll.offset();
        self.tabs.active_mut().show(page);
        self.after_navigation(offset);
        SceneReaction::Redraw
    }

    fn target_at(&mut self, x: f32, y: f32) -> Option<Url> {
        if y < CHROME_HEIGHT {
            return None;
        }
        let tab = self.tabs.active();
        let content = tab.content()?;
        let layout = content.layout.as_ref()?;
        let point = Point::new(x, y - CHROME_HEIGHT + self.scroll.offset());
        link_target(
            layout,
            &content.document,
            &mut self.text_engine,
            tab.url(),
            point,
        )
    }

    fn click(&mut self, x: f32, y: f32) -> SceneReaction {
        let point = Point::new(x, y);
        let count = self.tabs.len();
        if let Some(target) = hit_tab_strip(point, count, self.viewport_width) {
            return match target {
                TabStripTarget::Tab(index) => self.switch_to(index),
                TabStripTarget::Close(index) => self.close_tab(index),
                TabStripTarget::New => self.new_tab(),
                TabStripTarget::Empty => SceneReaction::Idle,
            };
        }
        if let Some(target) = hit_toolbar(point, self.viewport_width) {
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
                    self.address
                        .set_text(&address_text(self.tabs.active().url()));
                    self.address.blur();
                    return SceneReaction::Redraw;
                }
                AddressOutcome::Ignored => {}
            }
        }
        let count = self.tabs.len();
        let active = self.tabs.active_index();
        match key {
            KeyInput::GoBack => self.travel(false),
            KeyInput::GoForward => self.travel(true),
            KeyInput::Reload => self.reload(),
            KeyInput::FocusAddress => {
                self.address.focus();
                SceneReaction::Redraw
            }
            KeyInput::NewTab => self.new_tab(),
            KeyInput::CloseTab => self.close_tab(active),
            KeyInput::NextTab => self.switch_to((active + 1) % count),
            KeyInput::PreviousTab => self.switch_to((active + count - 1) % count),
            KeyInput::ToggleMemoryPanel => {
                self.shows_memory = !self.shows_memory;
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
        let Some(content) = self.tabs.active_mut().content_mut() else {
            return;
        };
        let is_current = content
            .layout
            .as_ref()
            .is_some_and(|layout| layout.width == width);
        if is_current {
            return;
        }
        let tree = layout_page(
            &content.document,
            &content.styles,
            &content.images,
            &mut self.text_engine,
            width as f32,
        );
        content.layout = Some(LayoutCache {
            width,
            height: tree.height,
            background: tree.background,
            list: build_display_list(&tree.root),
            root: tree.root,
        });
    }

    fn metrics(&self) -> Option<ScrollMetrics> {
        let layout = self.tabs.active().content()?.layout.as_ref()?;
        Some(ScrollMetrics::new(layout.height, self.viewport_height))
    }

    fn tab_strip_state(&self) -> TabStripState {
        TabStripState {
            titles: self.tabs.tabs().iter().map(|tab| tab.title()).collect(),
            discarded: self
                .tabs
                .tabs()
                .iter()
                .map(|tab| tab.is_discarded())
                .collect(),
            active: self.tabs.active_index(),
        }
    }
}

impl RenderScene for PageScene {
    fn render(&mut self, canvas: &mut Canvas) {
        self.viewport_width = canvas.width() as f32;
        self.viewport_height = (canvas.height() as f32 - CHROME_HEIGHT).max(0.0);
        self.layout_for(canvas.width());
        if let Some(metrics) = self.metrics() {
            if let Some(scroll) = self.pending_scroll.take() {
                self.scroll.scroll_to(scroll, metrics);
            }
            self.scroll.clamp(metrics);
            self.paint_page(canvas, metrics);
        } else {
            clear(canvas, Rgba::opaque(255, 255, 255));
        }
        let strip = self.tab_strip_state();
        draw_tab_strip(canvas, &mut self.text_engine, &strip);
        let toolbar = ToolbarState {
            field: &self.address,
            can_go_back: self.tabs.active().history().can_go_back(),
            can_go_forward: self.tabs.active().history().can_go_forward(),
        };
        draw_toolbar(canvas, &mut self.text_engine, &toolbar);
        if self.shows_memory {
            let process = self.meter.sample_if_stale(MEMORY_REFRESH);
            let summary = memory_summary(process, self.tabs.tabs(), shared_image_bytes());
            draw_memory_bar(canvas, &mut self.text_engine, &summary);
        }
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
        Some(format!("{} - firerust", self.tabs.active().title()))
    }

    fn pointer(&self) -> PointerKind {
        if self.is_over_link {
            PointerKind::Hand
        } else {
            PointerKind::Arrow
        }
    }
}

impl PageScene {
    fn paint_page(&mut self, canvas: &mut Canvas, metrics: ScrollMetrics) {
        let Some(layout) = self
            .tabs
            .active()
            .content()
            .and_then(|content| content.layout.as_ref())
        else {
            return;
        };
        clear(canvas, layout.background);
        paint_display_list(
            canvas,
            &mut self.text_engine,
            &layout.list,
            self.scroll.offset() - CHROME_HEIGHT,
        );
        if let Some(thumb) = scrollbar_thumb(self.scroll.offset(), metrics, self.viewport_width) {
            let placed = Rect::from_xywh(
                thumb.origin.x,
                thumb.origin.y + CHROME_HEIGHT,
                thumb.size.width,
                thumb.size.height,
            );
            fill_rect(canvas, placed, Rgba::new(0, 0, 0, 90));
        }
    }
}
