use std::time::Duration;

use fr_dom::node::node_id::NodeId;
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_layout::control::control_kind::ControlKind;
use fr_layout::page::layout_page::layout_page_with_focus;
use fr_mem::meter::memory_meter::MemoryMeter;
use fr_net::load::request::Request;
use fr_page::images::shared_image_cache::shared_image_bytes;
use fr_page::load::blank_page::{BLANK_ADDRESS, blank_page};
use fr_page::load::build_page::build_prefetched;
use fr_page::load::error_page::error_page;
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
use crate::scenes::click_control::click_control;
use crate::scenes::control_action::ControlAction;
use crate::scenes::control_at::control_at;
use crate::scenes::control_kind_in::control_kind_in;
use crate::scenes::dispatch_event_to::dispatch_event_to;
use crate::scenes::dispatch_page_click::dispatch_page_click;
use crate::scenes::focus_key::FocusKey;
use crate::scenes::form_focus::FormFocus;
use crate::scenes::form_for::form_for;
use crate::scenes::form_request::form_request;
use crate::scenes::link_target::link_target;
use crate::scenes::load_target::LoadTarget;
use crate::scenes::neighbour_in::neighbour_in;
use crate::scenes::pending_load::PendingLoad;
use crate::scenes::refresh_active::refresh_active;
use crate::scenes::request_for::request_for;
use crate::scenes::run_due_timers::run_due_timers;
use crate::scenes::timer_delay::timer_delay;
use crate::toolbar::draw_load_bar::draw_load_bar;
use crate::toolbar::draw_memory_bar::draw_memory_bar;
use crate::toolbar::draw_tab_strip::draw_tab_strip;
use crate::toolbar::draw_toolbar::draw_toolbar;
use crate::toolbar::tab_strip_state::TabStripState;
use crate::toolbar::toolbar_state::ToolbarState;

const MAX_LIVE_TABS: usize = 2;
const MEMORY_REFRESH: Duration = Duration::from_secs(1);
const LOAD_POLL: Duration = Duration::from_millis(33);

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
    form: FormFocus,
    pending: Option<PendingLoad>,
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
            form: FormFocus::new(),
            pending: None,
        }
    }

    pub fn open_first(&mut self, input: &str) {
        if let Some(request) = request_for(input) {
            self.begin_request(request, LoadTarget::Initial);
        }
    }

    pub fn add_background_tab(&mut self, page: Page) {
        self.tabs.open(page);
        self.tabs.activate(0);
        self.tabs.enforce_limit(MAX_LIVE_TABS);
    }

    fn remember_scroll(&mut self) {
        let offset = self.scroll.offset();
        self.tabs.active_mut().set_scroll(offset);
    }

    fn sync_active(&mut self) {
        let scroll = self.tabs.active().scroll();
        if self.tabs.active().is_discarded() {
            let url = self.tabs.active().url().clone();
            self.begin_request(Request::get(url), LoadTarget::Restore { scroll });
        }
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
        self.form.forget();
    }

    fn switch_to(&mut self, index: usize) -> SceneReaction {
        self.remember_scroll();
        self.pending = None;
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
        self.pending = None;
        self.tabs.open(page);
        self.sync_active();
        self.address.focus();
        SceneReaction::Redraw
    }

    fn close_tab(&mut self, index: usize) -> SceneReaction {
        self.remember_scroll();
        self.pending = None;
        if !self.tabs.close(index) {
            return SceneReaction::Idle;
        }
        self.sync_active();
        SceneReaction::Redraw
    }

    fn begin_request(&mut self, request: Request, target: LoadTarget) {
        let tab = self.tabs.active().id();
        self.address.set_text(&address_text(&request.url));
        self.pending = Some(PendingLoad::start(request, target, tab));
    }

    fn open(&mut self, input: &str) -> SceneReaction {
        if input.trim() == BLANK_ADDRESS {
            return match blank_page() {
                Some(page) => self.navigate(page),
                None => SceneReaction::Redraw,
            };
        }
        self.open_request(request_for(input))
    }

    fn open_request(&mut self, request: Option<Request>) -> SceneReaction {
        if let Some(request) = request {
            self.begin_request(request, LoadTarget::Navigate);
        }
        SceneReaction::Redraw
    }

    fn finish_load(&mut self) -> bool {
        let Some(pending) = self.pending.as_ref() else {
            return false;
        };
        let Some(outcome) = pending.poll() else {
            return false;
        };
        let Some(pending) = self.pending.take() else {
            return false;
        };
        if self.tabs.active().id() != pending.tab {
            return false;
        }
        let page = match outcome {
            Ok(prefetched) => build_prefetched(prefetched),
            Err(message) => {
                eprintln!("cannot open {}: {message}", pending.url);
                error_page(pending.url.clone(), &message)
            }
        };
        match pending.target {
            LoadTarget::Initial => {
                self.tabs
                    .active_mut()
                    .history_mut()
                    .replace_current(page.url.clone());
                self.tabs.active_mut().show(page);
                self.after_navigation(0.0);
            }
            LoadTarget::Navigate => {
                self.navigate(page);
            }
            LoadTarget::Travel { scroll } => {
                self.tabs.active_mut().show(page);
                self.after_navigation(scroll);
            }
            LoadTarget::Reload { scroll } => {
                self.tabs.active_mut().show(page);
                self.after_navigation(scroll);
            }
            LoadTarget::Restore { scroll } => {
                self.tabs.active_mut().restore(page);
                self.after_navigation(scroll);
                self.tabs.enforce_limit(MAX_LIVE_TABS);
            }
        }
        true
    }

    fn navigate(&mut self, page: Page) -> SceneReaction {
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
        self.begin_request(Request::get(url), LoadTarget::Travel { scroll });
        SceneReaction::Redraw
    }

    fn reload(&mut self) -> SceneReaction {
        let url = self.tabs.active().url().clone();
        let scroll = self.scroll.offset();
        self.begin_request(Request::get(url), LoadTarget::Reload { scroll });
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
            &content.document.borrow(),
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
        let link = self.target_at(x, y);
        let page_point = Point::new(x, y - CHROME_HEIGHT + self.scroll.offset());
        let control = control_at(&mut self.tabs, &mut self.text_engine, page_point);
        let allowed =
            dispatch_page_click(&mut self.tabs, &mut self.text_engine, page_point, (x, y));
        let changed = refresh_active(&mut self.tabs);
        let blurred = control != self.form.control() && self.form.blur(&mut self.tabs);
        if allowed {
            if let Some(control) = control {
                let action = click_control(&mut self.tabs, control);
                let reaction = self.apply_control_action(action);
                refresh_active(&mut self.tabs);
                return reaction;
            }
            if let Some(url) = link {
                return self.open(url.as_str());
            }
        }
        if changed || was_focused || blurred {
            SceneReaction::Redraw
        } else {
            SceneReaction::Idle
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
        match self.form.press(&mut self.tabs, key) {
            FocusKey::Edited => {
                refresh_active(&mut self.tabs);
                return SceneReaction::Redraw;
            }
            FocusKey::SubmitFrom(control) => return self.submit_from(control),
            FocusKey::Activate(control) => {
                let action = click_control(&mut self.tabs, control);
                let reaction = self.apply_control_action(action);
                refresh_active(&mut self.tabs);
                return reaction;
            }
            FocusKey::Blurred => return SceneReaction::Redraw,
            FocusKey::Ignored => {}
        }
        let count = self.tabs.len();
        let active = self.tabs.active_index();
        match key {
            KeyInput::FocusNext => self.move_focus(true),
            KeyInput::FocusPrevious => self.move_focus(false),
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

    fn apply_control_action(&mut self, action: ControlAction) -> SceneReaction {
        match action {
            ControlAction::Focus(control) => {
                self.form.focus(&mut self.tabs, control);
                SceneReaction::Redraw
            }
            ControlAction::Changed(control) => {
                dispatch_event_to(&mut self.tabs, control, "input");
                dispatch_event_to(&mut self.tabs, control, "change");
                self.form.focus(&mut self.tabs, control);
                SceneReaction::Redraw
            }
            ControlAction::Submit { form, submitter } => self.submit_form(form, Some(submitter)),
            ControlAction::Nothing => SceneReaction::Idle,
        }
    }

    fn submit_from(&mut self, control: NodeId) -> SceneReaction {
        match form_for(&mut self.tabs, control) {
            Some(form) => self.submit_form(form, None),
            None => SceneReaction::Idle,
        }
    }

    fn submit_form(&mut self, form: NodeId, submitter: Option<NodeId>) -> SceneReaction {
        let allowed = dispatch_event_to(&mut self.tabs, form, "submit");
        refresh_active(&mut self.tabs);
        if !allowed {
            return SceneReaction::Redraw;
        }
        match form_request(&mut self.tabs, form, submitter) {
            Some(request) => self.open_request(Some(request)),
            None => SceneReaction::Idle,
        }
    }

    fn move_focus(&mut self, forward: bool) -> SceneReaction {
        let current = self.form.control();
        let Some(next) = neighbour_in(&mut self.tabs, current, forward) else {
            return SceneReaction::Idle;
        };
        self.form.focus(&mut self.tabs, next);
        SceneReaction::Redraw
    }

    fn is_over_clickable_control(&mut self, x: f32, y: f32) -> bool {
        if y < CHROME_HEIGHT {
            return false;
        }
        let page_point = Point::new(x, y - CHROME_HEIGHT + self.scroll.offset());
        let Some(control) = control_at(&mut self.tabs, &mut self.text_engine, page_point) else {
            return false;
        };
        !matches!(
            control_kind_in(&mut self.tabs, control),
            Some(ControlKind::Text | ControlKind::Password | ControlKind::TextArea) | None
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
        let focus = self.form.layout_focus();
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
        let tree = layout_page_with_focus(
            &content.document.borrow(),
            &content.styles,
            &content.images,
            &mut self.text_engine,
            width as f32,
            focus,
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
        if let Some(pending) = &self.pending {
            draw_load_bar(canvas, pending.started.elapsed());
        }
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
                self.is_over_link =
                    self.target_at(x, y).is_some() || self.is_over_clickable_control(x, y);
                SceneReaction::Idle
            }
            InputEvent::Character(character) if self.address.is_focused() => {
                self.address.insert(character);
                SceneReaction::Redraw
            }
            InputEvent::Character(character) => {
                if self.form.type_char(&mut self.tabs, character) {
                    refresh_active(&mut self.tabs);
                    SceneReaction::Redraw
                } else {
                    SceneReaction::Idle
                }
            }
            InputEvent::Key(key) => self.press(key),
            InputEvent::Wheel { .. } => self.scroll_for(event),
        }
    }

    fn next_wakeup(&mut self) -> Option<Duration> {
        let timers = timer_delay(&mut self.tabs);
        if self.pending.is_some() {
            return Some(timers.map_or(LOAD_POLL, |delay| delay.min(LOAD_POLL)));
        }
        timers
    }

    fn tick(&mut self) -> SceneReaction {
        let loaded = self.finish_load();
        run_due_timers(&mut self.tabs);
        if refresh_active(&mut self.tabs) || loaded || self.pending.is_some() {
            SceneReaction::Redraw
        } else {
            SceneReaction::Idle
        }
    }

    fn title(&self) -> Option<String> {
        if let Some(pending) = &self.pending {
            return Some(format!("Loading {} - firerust", pending.url));
        }
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
