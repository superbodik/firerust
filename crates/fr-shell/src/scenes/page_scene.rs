use std::collections::HashMap;
use std::time::{Duration, Instant};

use fr_bookmarks::storage::bookmark_book::{BookmarkBook, BookmarkChange};
use fr_clipboard::text::clean_pasted_text::clean_pasted_text;
use fr_clipboard::text::read_text::read_text;
use fr_clipboard::text::write_text::write_text;
use fr_dom::node::node_id::NodeId;
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_layout::anchor::anchor_top::anchor_top;
use fr_layout::control::control_kind::ControlKind;
use fr_layout::page::layout_page::layout_page_with_focus;
use fr_mem::meter::memory_meter::MemoryMeter;
use fr_net::address::percent_decode::percent_decode;
use fr_net::load::request::Request;
use fr_net::load::response::Response;
use fr_page::images::shared_image_cache::shared_image_bytes;
use fr_page::link::classify_link::classify_link;
use fr_page::link::link_action::LinkAction;
use fr_page::link::same_document::is_same_document;
use fr_page::load::blank_page::blank_page_for;
use fr_page::load::build_page::build_prefetched;
use fr_page::load::error_page::error_page_for;
use fr_page::load::page::Page;
use fr_page::prefetch::fetched::Fetched;
use fr_paint::build::build_display_list::build_display_list;
use fr_paint::paint::paint_display_list::paint_display_list;
use fr_passwords::model::credential::Credential;
use fr_passwords::storage::password_vault::PasswordVault;
use fr_raster::draw::clear::clear;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_settings::model::search_engine::SearchEngine;
use fr_settings::model::settings::Settings;
use fr_settings::model::theme_choice::ThemeChoice;
use fr_settings::storage::load_settings::load_settings;
use fr_settings::storage::save_settings::save_settings;
use fr_settings::storage::settings_path::settings_path;
use fr_tab::model::layout_cache::LayoutCache;
use fr_tab::set::tab_set::TabSet;
use fr_tab::summary::memory_summary::memory_summary;
use fr_text::engine::text_engine::TextEngine;
use fr_theme::colors::palette::Palette;
use fr_theme::colors::palette_for::palette_for;
use fr_theme::kind::theme_kind::ThemeKind;
use fr_ui::address::address_field::AddressField;
use fr_ui::address::address_outcome::AddressOutcome;
use fr_ui::address::apply_address_key::apply_address_key;
use fr_ui::hover::chrome_hover::ChromeHover;
use fr_ui::hover::hover_chrome::hover_chrome;
use fr_ui::menu::hit_menu::hit_menu;
use fr_ui::menu::menu_hit::MenuHit;
use fr_ui::menu::menu_kind::MenuKind;
use fr_ui::menu::menu_target::MenuTarget;
use fr_ui::menu::start_button_rect::start_button_rect;
use fr_ui::scroll::scroll_metrics::ScrollMetrics;
use fr_ui::scroll::scroll_state::ScrollState;
use fr_ui::scroll::scrollbar_thumb::scrollbar_thumb;
use fr_ui::tabs::hit_tab_strip::hit_tab_strip;
use fr_ui::tabs::tab_strip_target::TabStripTarget;
use fr_ui::tabs::window_control::WindowControl;
use fr_ui::toolbar::hit_toolbar::hit_toolbar;
use fr_ui::toolbar::toolbar_metrics::CHROME_HEIGHT;
use fr_ui::toolbar::toolbar_target::ToolbarTarget;
use fr_ui::zoom::zoom_level::ZoomLevel;
use fr_window::input::input_event::InputEvent;
use fr_window::input::key_input::KeyInput;
use fr_window::scene::pointer_kind::PointerKind;
use fr_window::scene::render_scene::RenderScene;
use fr_window::scene::scene_reaction::SceneReaction;
use url::Url;

use crate::internal::internal_context::InternalContext;
use crate::internal::internal_page::internal_page;
use crate::internal::query_value::query_value;
use crate::scenes::address_text::address_text;
use crate::scenes::autofill_logins::autofill_logins;
use crate::scenes::click_control::click_control;
use crate::scenes::control_action::ControlAction;
use crate::scenes::control_at::control_at;
use crate::scenes::control_kind_in::control_kind_in;
use crate::scenes::dispatch_event_to::dispatch_event_to;
use crate::scenes::dispatch_page_click::dispatch_page_click;
use crate::scenes::finish_download::finish_download;
use crate::scenes::focus_key::FocusKey;
use crate::scenes::forced_theme::forced_theme;
use crate::scenes::form_focus::FormFocus;
use crate::scenes::form_for::form_for;
use crate::scenes::form_request::form_request;
use crate::scenes::invalidate_layout::invalidate_layout;
use crate::scenes::launch_external::launch_external;
use crate::scenes::launch_new_window::launch_new_window;
use crate::scenes::link_target::link_target;
use crate::scenes::load_target::LoadTarget;
use crate::scenes::login_to_save::login_to_save;
use crate::scenes::neighbour_in::neighbour_in;
use crate::scenes::notice::Notice;
use crate::scenes::pending_load::PendingLoad;
use crate::scenes::refresh_active::refresh_active;
use crate::scenes::request_for::request_for;
use crate::scenes::run_due_timers::run_due_timers;
use crate::scenes::tab_icon::tab_icon;
use crate::scenes::theme_for::theme_for;
use crate::scenes::timer_delay::timer_delay;
use crate::scenes::zoom_key::zoom_key;
use crate::toolbar::draw_link_status::draw_link_status;
use crate::toolbar::draw_load_bar::draw_load_bar;
use crate::toolbar::draw_memory_bar::{MEMORY_BAR_HEIGHT, draw_memory_bar};
use crate::toolbar::draw_menu_popup::{MenuView, draw_menu_popup};
use crate::toolbar::draw_notice::draw_notice;
use crate::toolbar::draw_start_button::draw_start_button;
use crate::toolbar::draw_tab_strip::draw_tab_strip;
use crate::toolbar::draw_toolbar::draw_toolbar;
use crate::toolbar::security_badge::SecurityBadge;
use crate::toolbar::tab_strip_state::TabStripState;
use crate::toolbar::toolbar_state::ToolbarState;

const MAX_LIVE_TABS: usize = 2;
const MEMORY_REFRESH: Duration = Duration::from_secs(1);
const DOUBLE_CLICK: Duration = Duration::from_millis(400);
const LOAD_POLL: Duration = Duration::from_millis(33);
const NOTICE_LIFETIME: Duration = Duration::from_secs(3);
const SAVE_PROMPT_LIFETIME: Duration = Duration::from_secs(10);
const DOWNLOAD_NOTICE_LIFETIME: Duration = Duration::from_secs(6);
const MAX_VISITED: usize = 300;

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
    zooms: HashMap<String, ZoomLevel>,
    bookmarks: BookmarkBook,
    notice: Option<Notice>,
    vault: PasswordVault,
    pending_login: Option<Credential>,
    theme: ThemeKind,
    forced_theme: Option<ThemeKind>,
    settings: Settings,
    system_dark: bool,
    hover: ChromeHover,
    maximized: bool,
    last_strip_press: Option<Instant>,
    pending_fragment: Option<String>,
    hovered_link: Option<String>,
    logs_frames: bool,
    popup: Option<MenuKind>,
    popup_hover: Option<MenuTarget>,
    start_hover: bool,
}

impl PageScene {
    pub fn new(page: Page) -> Self {
        let forced_theme = forced_theme();
        let settings = settings_path()
            .map(|path| load_settings(&path))
            .unwrap_or_default();
        let mut address = AddressField::new();
        address.set_text(&address_text(&page.url));
        let mut scene = Self {
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
            zooms: HashMap::new(),
            bookmarks: BookmarkBook::open(),
            notice: None,
            vault: PasswordVault::open(),
            pending_login: None,
            theme: theme_for(forced_theme, settings.theme, false),
            forced_theme,
            settings,
            system_dark: false,
            hover: ChromeHover::Nothing,
            maximized: false,
            last_strip_press: None,
            pending_fragment: None,
            hovered_link: None,
            logs_frames: std::env::var_os("FIRERUST_FRAME_LOG").is_some(),
            popup: None,
            popup_hover: None,
            start_hover: false,
        };
        if scene.theme != ThemeKind::Light {
            scene.rebuild_internal_pages();
        }
        scene
    }

    pub fn open_first(&mut self, input: &str) {
        if let Some(request) = request_for(input, self.settings.search) {
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
        self.hovered_link = None;
        self.popup = None;
        self.popup_hover = None;
        self.form.forget();
        self.pending_fragment = if scroll <= 0.0 {
            self.tabs.active().url().fragment().map(percent_decode)
        } else {
            None
        };
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
        let Some(page) = blank_page_for(self.theme) else {
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
        if request.url.scheme() == "about" && request.url.path() == "search" {
            let typed = query_value(&request.url, "q").unwrap_or_default();
            if let Some(search) = request_for(&typed, self.settings.search) {
                self.begin_request(search, target);
            }
            return;
        }
        if request.url.scheme() == "about" {
            self.pending = None;
            self.apply_requested_settings(&request.url);
            let visited = self.visited_urls();
            let mut context = InternalContext {
                bookmarks: &mut self.bookmarks,
                vault: &mut self.vault,
                settings: &self.settings,
                is_theme_forced: self.forced_theme.is_some(),
                theme: self.theme,
                visited: &visited,
            };
            if let Some(page) = internal_page(&request.url, &mut context) {
                self.apply_page(page, target);
            }
            return;
        }
        let tab = self.tabs.active().id();
        self.address.set_text(&address_text(&request.url));
        self.pending = Some(PendingLoad::start(request, target, tab));
    }

    fn open(&mut self, input: &str) -> SceneReaction {
        self.open_request(request_for(input, self.settings.search))
    }

    fn open_request(&mut self, request: Option<Request>) -> SceneReaction {
        if let Some(request) = request {
            self.begin_request(request, LoadTarget::Navigate);
        }
        SceneReaction::Redraw
    }

    fn apply_page(&mut self, page: Page, target: LoadTarget) {
        match target {
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
            LoadTarget::Travel { scroll } | LoadTarget::Reload { scroll } => {
                self.tabs.active_mut().show(page);
                self.after_navigation(scroll);
            }
            LoadTarget::Restore { scroll } => {
                self.tabs.active_mut().restore(page);
                self.after_navigation(scroll);
                self.tabs.enforce_limit(MAX_LIVE_TABS);
            }
        }
        if autofill_logins(&mut self.tabs, &self.vault) {
            refresh_active(&mut self.tabs);
        }
        self.take_script_navigation();
    }

    fn take_script_navigation(&mut self) -> Option<SceneReaction> {
        let request = self.tabs.active_mut().content_mut()?.take_navigation()?;
        let base = self.tabs.active().url().clone();
        let target = request.in_new_tab.then_some("_blank");
        let action = classify_link(&base, &request.address, target)?;
        if let LinkAction::Open {
            url,
            in_new_tab: false,
        } = &action
            && is_same_document(url, &base)
            && url.fragment().is_none()
        {
            return None;
        }
        Some(self.follow_link(action, false))
    }

    fn remember_login(&mut self, form: NodeId) {
        let Some(login) = login_to_save(&mut self.tabs, &self.vault, form) else {
            return;
        };
        let message = format!(
            "Save the password for {} on {}? Press Ctrl+S",
            if login.username.is_empty() {
                "this login"
            } else {
                login.username.as_str()
            },
            login.origin
        );
        self.pending_login = Some(login);
        self.notice = Some(Notice::new(&message, SAVE_PROMPT_LIFETIME));
    }

    fn save_password(&mut self) -> SceneReaction {
        let message = match self.pending_login.take() {
            Some(login) => {
                self.vault.save_login(login);
                "Password saved (Ctrl+P shows them)"
            }
            None => "No new password to save: sign in on a site first",
        };
        self.notice = Some(Notice::new(message, NOTICE_LIFETIME));
        SceneReaction::Redraw
    }

    fn open_passwords(&mut self) -> SceneReaction {
        self.open("about:passwords")
    }

    fn toggle_bookmark(&mut self) -> SceneReaction {
        let url = self.tabs.active().url().to_string();
        let title = self.tabs.active().title();
        let message = match self.bookmarks.toggle(&title, &url) {
            BookmarkChange::Added => "Bookmark added (Ctrl+B shows them)",
            BookmarkChange::Removed => "Bookmark removed",
            BookmarkChange::Unsupported => "Only web pages can be bookmarked",
        };
        self.notice = Some(Notice::new(message, NOTICE_LIFETIME));
        SceneReaction::Redraw
    }

    fn open_bookmarks(&mut self) -> SceneReaction {
        self.open("about:bookmarks")
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
            Ok(Fetched::Page(prefetched)) => build_prefetched(prefetched),
            Ok(Fetched::Download(response)) => {
                self.finish_download(&response, pending.url.scheme() == "file");
                return true;
            }
            Err(message) => {
                eprintln!("cannot open {}: {message}", pending.url);
                error_page_for(pending.url.clone(), &message, self.theme)
            }
        };
        self.apply_page(page, pending.target);
        true
    }

    fn finish_download(&mut self, response: &Response, requested_from_disk: bool) {
        self.address
            .set_text(&address_text(self.tabs.active().url()));
        let message = finish_download(response, requested_from_disk);
        self.notice = Some(Notice::new(&message, DOWNLOAD_NOTICE_LIFETIME));
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
        let stays_here =
            !self.tabs.active().is_discarded() && is_same_document(&url, self.tabs.active().url());
        if stays_here {
            self.tabs.active_mut().set_url(url);
            self.address
                .set_text(&address_text(self.tabs.active().url()));
            self.pending_scroll = Some(scroll);
            return SceneReaction::Redraw;
        }
        self.begin_request(Request::get(url), LoadTarget::Travel { scroll });
        SceneReaction::Redraw
    }

    fn reload_or_stop(&mut self) -> SceneReaction {
        if self.pending.take().is_some() {
            return SceneReaction::Redraw;
        }
        self.reload()
    }

    fn palette(&self) -> &'static Palette {
        palette_for(self.theme)
    }

    fn reload(&mut self) -> SceneReaction {
        let url = self.tabs.active().url().clone();
        let scroll = self.scroll.offset();
        self.begin_request(Request::get(url), LoadTarget::Reload { scroll });
        SceneReaction::Redraw
    }

    fn press_at(&mut self, x: f32, y: f32) -> SceneReaction {
        let target = hit_tab_strip(Point::new(x, y), self.tabs.len(), self.viewport_width);
        if target != Some(TabStripTarget::Empty) {
            self.last_strip_press = None;
            return SceneReaction::Idle;
        }
        let is_double = self
            .last_strip_press
            .is_some_and(|earlier| earlier.elapsed() < DOUBLE_CLICK);
        if is_double {
            self.last_strip_press = None;
            return SceneReaction::ToggleMaximize;
        }
        self.last_strip_press = Some(Instant::now());
        SceneReaction::DragWindow
    }

    fn visited_urls(&self) -> Vec<String> {
        let mut seen: Vec<String> = Vec::new();
        for tab in self.tabs.tabs() {
            for entry in tab.history().entries().iter().rev() {
                let text = entry.url.to_string();
                if entry.url.scheme() != "about" && !seen.contains(&text) {
                    seen.push(text);
                }
                if seen.len() >= MAX_VISITED {
                    return seen;
                }
            }
        }
        seen
    }

    fn rebuild_internal_pages(&mut self) {
        let internal: Vec<(usize, Url)> = self
            .tabs
            .tabs()
            .iter()
            .enumerate()
            .filter(|(_, tab)| tab.url().scheme() == "about" && !tab.is_discarded())
            .map(|(index, tab)| (index, tab.url().clone()))
            .collect();
        for (index, url) in internal {
            let visited = self.visited_urls();
            let mut context = InternalContext {
                bookmarks: &mut self.bookmarks,
                vault: &mut self.vault,
                settings: &self.settings,
                is_theme_forced: self.forced_theme.is_some(),
                theme: self.theme,
                visited: &visited,
            };
            let Some(page) = internal_page(&url, &mut context) else {
                continue;
            };
            if let Some(tab) = self.tabs.tab_mut(index) {
                tab.restore(page);
            }
        }
    }

    fn zoom(&self) -> ZoomLevel {
        let key = zoom_key(self.tabs.active().url());
        self.zooms.get(&key).copied().unwrap_or_default()
    }

    fn factor(&self) -> f32 {
        self.zoom().factor()
    }

    fn page_point(&self, x: f32, y: f32) -> Point {
        let factor = self.factor();
        Point::new(
            x / factor,
            (y - CHROME_HEIGHT + self.scroll.offset()) / factor,
        )
    }

    fn change_zoom(&mut self, change: fn(ZoomLevel) -> ZoomLevel) -> SceneReaction {
        let old = self.zoom();
        let new = change(old);
        if new == old {
            return SceneReaction::Idle;
        }
        let anchor = self.scroll.offset() / old.factor();
        let key = zoom_key(self.tabs.active().url());
        self.zooms.insert(key, new);
        invalidate_layout(&mut self.tabs);
        self.pending_scroll = Some(anchor * new.factor());
        SceneReaction::Redraw
    }

    fn target_at(&mut self, x: f32, y: f32) -> Option<LinkAction> {
        if y < CHROME_HEIGHT {
            return None;
        }
        let point = self.page_point(x, y);
        let tab = self.tabs.active();
        let content = tab.content()?;
        let layout = content.layout.as_ref()?;
        link_target(
            layout,
            &content.document.borrow(),
            &mut self.text_engine,
            tab.url(),
            point,
        )
    }

    fn canvas_height(&self) -> f32 {
        self.viewport_height + CHROME_HEIGHT
    }

    fn is_start_page(&self) -> bool {
        self.tabs.active().url().as_str() == "about:blank"
    }

    fn click_popup(&mut self, point: Point) -> Option<SceneReaction> {
        let height = self.canvas_height();
        if let Some(kind) = self.popup {
            return Some(match hit_menu(kind, point, self.viewport_width, height) {
                MenuHit::Target(target) => self.run_menu(target),
                MenuHit::Inert => SceneReaction::Idle,
                MenuHit::Outside => {
                    self.popup = None;
                    self.popup_hover = None;
                    SceneReaction::Redraw
                }
            });
        }
        if self.is_start_page() && start_button_rect(height).contains(point) {
            self.popup = Some(MenuKind::Start);
            return Some(SceneReaction::Redraw);
        }
        None
    }

    fn toggle_main_menu(&mut self) -> SceneReaction {
        self.popup = match self.popup {
            Some(MenuKind::Main) => None,
            _ => Some(MenuKind::Main),
        };
        self.popup_hover = None;
        SceneReaction::Redraw
    }

    fn run_menu(&mut self, target: MenuTarget) -> SceneReaction {
        if !matches!(
            target,
            MenuTarget::ZoomIn | MenuTarget::ZoomOut | MenuTarget::ZoomReset
        ) {
            self.popup = None;
            self.popup_hover = None;
        }
        match target {
            MenuTarget::NewTab => self.new_tab(),
            MenuTarget::NewWindow => self.open_new_window(),
            MenuTarget::History => self.open("about:history"),
            MenuTarget::Bookmarks => self.open_bookmarks(),
            MenuTarget::Downloads => self.open("about:downloads"),
            MenuTarget::Passwords => self.open_passwords(),
            MenuTarget::Settings => self.open("about:settings"),
            MenuTarget::Console => self.open("about:console"),
            MenuTarget::Pages => self.open("about:menu"),
            MenuTarget::Exit => SceneReaction::CloseWindow,
            MenuTarget::ZoomIn => self.change_zoom(ZoomLevel::zoom_in),
            MenuTarget::ZoomOut => self.change_zoom(ZoomLevel::zoom_out),
            MenuTarget::ZoomReset => self.change_zoom(|_| ZoomLevel::new()),
        }
    }

    fn open_new_window(&mut self) -> SceneReaction {
        let message = match launch_new_window() {
            Ok(()) => "Opening a new window",
            Err(error) => {
                eprintln!("cannot open a new window: {error}");
                "Cannot open a new window"
            }
        };
        self.notice = Some(Notice::new(message, NOTICE_LIFETIME));
        SceneReaction::Redraw
    }

    fn update_popup_hover(&mut self, point: Point) -> bool {
        let height = self.canvas_height();
        let (hover, start) = match self.popup {
            Some(kind) => match hit_menu(kind, point, self.viewport_width, height) {
                MenuHit::Target(target) => (Some(target), false),
                _ => (None, false),
            },
            None => (
                None,
                self.is_start_page() && start_button_rect(height).contains(point),
            ),
        };
        let changed = hover != self.popup_hover || start != self.start_hover;
        self.popup_hover = hover;
        self.start_hover = start;
        changed
    }

    fn click(&mut self, x: f32, y: f32) -> SceneReaction {
        let point = Point::new(x, y);
        if let Some(reaction) = self.click_popup(point) {
            return reaction;
        }
        let count = self.tabs.len();
        if let Some(target) = hit_tab_strip(point, count, self.viewport_width) {
            return match target {
                TabStripTarget::Tab(index) => self.switch_to(index),
                TabStripTarget::Close(index) => self.close_tab(index),
                TabStripTarget::New => self.new_tab(),
                TabStripTarget::Control(WindowControl::Minimize) => SceneReaction::Minimize,
                TabStripTarget::Control(WindowControl::Maximize) => SceneReaction::ToggleMaximize,
                TabStripTarget::Control(WindowControl::Close) => SceneReaction::CloseWindow,
                TabStripTarget::Empty => SceneReaction::Idle,
            };
        }
        if let Some(target) = hit_toolbar(point, self.viewport_width) {
            return match target {
                ToolbarTarget::Back => self.travel(false),
                ToolbarTarget::Forward => self.travel(true),
                ToolbarTarget::Reload => self.reload_or_stop(),
                ToolbarTarget::Star => self.toggle_bookmark(),
                ToolbarTarget::Menu => self.toggle_main_menu(),
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
        let page_point = self.page_point(x, y);
        let control = control_at(&mut self.tabs, &mut self.text_engine, page_point);
        let factor = self.factor();
        let allowed = dispatch_page_click(
            &mut self.tabs,
            &mut self.text_engine,
            page_point,
            (x / factor, (y - CHROME_HEIGHT) / factor),
        );
        let changed = refresh_active(&mut self.tabs);
        if let Some(reaction) = self.take_script_navigation() {
            return reaction;
        }
        let blurred = control != self.form.control() && self.form.blur(&mut self.tabs);
        if allowed {
            if let Some(control) = control {
                let action = click_control(&mut self.tabs, control);
                let reaction = self.apply_control_action(action);
                refresh_active(&mut self.tabs);
                return reaction;
            }
            if let Some(action) = link {
                return self.follow_link(action, false);
            }
        }
        if changed || was_focused || blurred {
            SceneReaction::Redraw
        } else {
            SceneReaction::Idle
        }
    }

    fn refresh_theme(&mut self) -> SceneReaction {
        let theme = theme_for(self.forced_theme, self.settings.theme, self.system_dark);
        if theme == self.theme {
            return SceneReaction::Idle;
        }
        self.theme = theme;
        self.rebuild_internal_pages();
        SceneReaction::Redraw
    }

    fn apply_requested_settings(&mut self, url: &Url) {
        if url.path() != "settings" {
            return;
        }
        let theme = query_value(url, "theme").and_then(|key| ThemeChoice::from_key(&key));
        let search = query_value(url, "search").and_then(|key| SearchEngine::from_key(&key));
        if theme.is_none() && search.is_none() {
            return;
        }
        if let Some(choice) = theme {
            self.settings.theme = choice;
        }
        if let Some(engine) = search {
            self.settings.search = engine;
        }
        if let Some(path) = settings_path()
            && let Err(error) = save_settings(&path, &self.settings)
        {
            eprintln!("cannot save the settings: {error}");
        }
        self.refresh_theme();
    }

    fn secondary_click(&mut self, x: f32, y: f32) -> SceneReaction {
        let count = self.tabs.len();
        match hit_tab_strip(Point::new(x, y), count, self.viewport_width) {
            Some(TabStripTarget::Tab(index) | TabStripTarget::Close(index)) => {
                return self.close_tab(index);
            }
            Some(_) => return SceneReaction::Idle,
            None => {}
        }
        match self.target_at(x, y) {
            Some(action) => self.follow_link(action, true),
            None => SceneReaction::Idle,
        }
    }

    fn follow_link(&mut self, action: LinkAction, force_new_tab: bool) -> SceneReaction {
        match action {
            LinkAction::Open { url, in_new_tab } => {
                let request = Request::get(url);
                if in_new_tab || force_new_tab {
                    self.open_in_new_tab(request)
                } else {
                    self.open_request(Some(request))
                }
            }
            LinkAction::Scroll { fragment } => self.scroll_to_fragment(&fragment),
            LinkAction::External { url } => self.open_external(&url),
        }
    }

    fn open_in_new_tab(&mut self, request: Request) -> SceneReaction {
        let Some(page) = blank_page_for(self.theme) else {
            return SceneReaction::Idle;
        };
        self.remember_scroll();
        self.pending = None;
        self.tabs.open(page);
        self.sync_active();
        self.begin_request(request, LoadTarget::Initial);
        SceneReaction::Redraw
    }

    fn open_external(&mut self, url: &Url) -> SceneReaction {
        let message = match launch_external(url) {
            Ok(()) => "Opening it in another app",
            Err(error) => {
                eprintln!("cannot open {url}: {error}");
                "Cannot open this link"
            }
        };
        self.notice = Some(Notice::new(message, NOTICE_LIFETIME));
        SceneReaction::Redraw
    }

    fn fragment_offset(&self, fragment: &str) -> Option<f32> {
        let content = self.tabs.active().content()?;
        let layout = content.layout.as_ref()?;
        let found = anchor_top(&layout.root, &content.document.borrow(), fragment);
        match found {
            Some(top) => Some(top),
            None if fragment.is_empty() || fragment.eq_ignore_ascii_case("top") => Some(0.0),
            None => None,
        }
    }

    fn scroll_to_fragment(&mut self, fragment: &str) -> SceneReaction {
        let Some(offset) = self.fragment_offset(fragment) else {
            return SceneReaction::Idle;
        };
        let mut url = self.tabs.active().url().clone();
        url.set_fragment((!fragment.is_empty()).then_some(fragment));
        let scroll = self.scroll.offset();
        let tab = self.tabs.active_mut();
        tab.history_mut().remember_scroll(scroll);
        tab.history_mut().visit(url.clone());
        tab.set_url(url);
        self.address
            .set_text(&address_text(self.tabs.active().url()));
        self.pending_scroll = Some(offset * self.factor());
        self.hovered_link = None;
        SceneReaction::Redraw
    }

    fn clipboard_key(&mut self, key: KeyInput) -> SceneReaction {
        if self.address.is_focused() {
            return self.clipboard_in_address(key);
        }
        if self.form.control().is_none() {
            return SceneReaction::Idle;
        }
        match key {
            KeyInput::Paste => {
                let Some(text) = read_text() else {
                    return SceneReaction::Idle;
                };
                if self.form.paste(&mut self.tabs, &clean_pasted_text(&text)) {
                    refresh_active(&mut self.tabs);
                    return SceneReaction::Redraw;
                }
            }
            KeyInput::Copy | KeyInput::Cut => {
                if let Some(text) = self.form.current_text() {
                    write_text(&text);
                }
            }
            _ => {}
        }
        SceneReaction::Idle
    }

    fn clipboard_in_address(&mut self, key: KeyInput) -> SceneReaction {
        match key {
            KeyInput::Paste => {
                let Some(text) = read_text() else {
                    return SceneReaction::Idle;
                };
                self.address.paste(&clean_pasted_text(&text));
                SceneReaction::Redraw
            }
            KeyInput::Copy => {
                if let Some(text) = self.address.copy_text() {
                    write_text(&text);
                }
                SceneReaction::Idle
            }
            KeyInput::Cut => match self.address.cut_text() {
                Some(text) => {
                    write_text(&text);
                    SceneReaction::Redraw
                }
                None => SceneReaction::Idle,
            },
            _ => {
                self.address.select_all();
                SceneReaction::Redraw
            }
        }
    }

    fn press(&mut self, key: KeyInput) -> SceneReaction {
        if key == KeyInput::Escape && self.popup.is_some() {
            self.popup = None;
            self.popup_hover = None;
            return SceneReaction::Redraw;
        }
        if matches!(
            key,
            KeyInput::Paste | KeyInput::Copy | KeyInput::Cut | KeyInput::SelectAll
        ) {
            return self.clipboard_key(key);
        }
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
            KeyInput::NewWindow => self.open_new_window(),
            KeyInput::OpenHistory => self.open("about:history"),
            KeyInput::OpenDownloads => self.open("about:downloads"),
            KeyInput::CloseTab => self.close_tab(active),
            KeyInput::NextTab => self.switch_to((active + 1) % count),
            KeyInput::PreviousTab => self.switch_to((active + count - 1) % count),
            KeyInput::SavePassword => self.save_password(),
            KeyInput::OpenPasswords => self.open_passwords(),
            KeyInput::ToggleBookmark => self.toggle_bookmark(),
            KeyInput::OpenBookmarks => self.open_bookmarks(),
            KeyInput::ZoomIn => self.change_zoom(ZoomLevel::zoom_in),
            KeyInput::ZoomOut => self.change_zoom(ZoomLevel::zoom_out),
            KeyInput::ZoomReset => self.change_zoom(|_| ZoomLevel::new()),
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
        self.remember_login(form);
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
        let page_point = self.page_point(x, y);
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
            self.hovered_link = None;
            SceneReaction::Redraw
        } else {
            SceneReaction::Idle
        }
    }

    fn layout_for(&mut self, width: u32) {
        let focus = self.form.layout_focus();
        let layout_width = (width as f32 / self.factor()).floor().max(1.0) as u32;
        let Some(content) = self.tabs.active_mut().content_mut() else {
            return;
        };
        let is_current = content
            .layout
            .as_ref()
            .is_some_and(|layout| layout.width == layout_width);
        if is_current {
            return;
        }
        let tree = layout_page_with_focus(
            &content.document.borrow(),
            &content.styles,
            &content.images,
            &mut self.text_engine,
            layout_width as f32,
            focus,
        );
        content.layout = Some(LayoutCache {
            width: layout_width,
            height: tree.height,
            background: tree.background,
            list: build_display_list(&tree.root),
            root: tree.root,
        });
    }

    fn metrics(&self) -> Option<ScrollMetrics> {
        let layout = self.tabs.active().content()?.layout.as_ref()?;
        Some(ScrollMetrics::new(
            layout.height * self.factor(),
            self.viewport_height,
        ))
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
            icons: self
                .tabs
                .tabs()
                .iter()
                .map(|tab| tab_icon(tab.url()))
                .collect(),
            active: self.tabs.active_index(),
            hover: self.hover,
            maximized: self.maximized,
        }
    }
}

impl RenderScene for PageScene {
    fn render(&mut self, canvas: &mut Canvas) {
        let frame_started = Instant::now();
        self.viewport_width = canvas.width() as f32;
        self.viewport_height = (canvas.height() as f32 - CHROME_HEIGHT).max(0.0);
        self.layout_for(canvas.width());
        let layout_time = frame_started.elapsed();
        let mut paint_time = Duration::ZERO;
        if let Some(metrics) = self.metrics() {
            if let Some(fragment) = self.pending_fragment.take()
                && let Some(offset) = self.fragment_offset(&fragment)
            {
                self.pending_scroll = Some(offset * self.factor());
            }
            if let Some(scroll) = self.pending_scroll.take() {
                self.scroll.scroll_to(scroll, metrics);
            }
            self.scroll.clamp(metrics);
            let paint_started = Instant::now();
            self.paint_page(canvas, metrics);
            paint_time = paint_started.elapsed();
        } else {
            clear(canvas, Rgba::opaque(255, 255, 255));
        }
        let palette = self.palette();
        let strip = self.tab_strip_state();
        let url = self.tabs.active().url().clone();
        let toolbar = ToolbarState {
            field: &self.address,
            can_go_back: self.tabs.active().history().can_go_back(),
            can_go_forward: self.tabs.active().history().can_go_forward(),
            is_loading: self.pending.is_some(),
            is_bookmarked: self.bookmarks.contains(url.as_str()),
            badge: SecurityBadge::of(&url),
            hover: self.hover,
        };
        draw_toolbar(canvas, &mut self.text_engine, palette, &toolbar);
        draw_tab_strip(canvas, &mut self.text_engine, palette, &strip);
        if let Some(pending) = &self.pending {
            draw_load_bar(canvas, palette, pending.started.elapsed());
        }
        let mut lift = 0.0;
        if self.shows_memory {
            let process = self.meter.sample_if_stale(MEMORY_REFRESH);
            let summary = memory_summary(process, self.tabs.tabs(), shared_image_bytes());
            draw_memory_bar(canvas, &mut self.text_engine, palette, &summary);
            lift = MEMORY_BAR_HEIGHT;
        }
        if let Some(text) = &self.hovered_link {
            draw_link_status(canvas, &mut self.text_engine, palette, text, lift);
        }
        if let Some(notice) = &self.notice {
            draw_notice(
                canvas,
                &mut self.text_engine,
                palette,
                &notice.message,
                lift,
            );
        }
        if self.is_start_page() {
            let is_lit = self.start_hover || self.popup == Some(MenuKind::Start);
            draw_start_button(canvas, &mut self.text_engine, palette, is_lit);
        }
        if let Some(kind) = self.popup {
            let view = MenuView {
                kind,
                hover: self.popup_hover,
                zoom_percent: u32::from(self.zoom().percent()),
            };
            draw_menu_popup(canvas, &mut self.text_engine, palette, &view);
        }
        if self.logs_frames {
            eprintln!(
                "frame {:.1} ms (layout {:.1}, page {:.1})",
                frame_started.elapsed().as_secs_f32() * 1000.0,
                layout_time.as_secs_f32() * 1000.0,
                paint_time.as_secs_f32() * 1000.0
            );
        }
    }

    fn set_dark(&mut self, dark: bool) -> SceneReaction {
        self.system_dark = dark;
        self.refresh_theme()
    }

    fn set_maximized(&mut self, maximized: bool) -> SceneReaction {
        if maximized == self.maximized {
            return SceneReaction::Idle;
        }
        self.maximized = maximized;
        SceneReaction::Redraw
    }

    fn handle(&mut self, event: InputEvent) -> SceneReaction {
        match event {
            InputEvent::Click { x, y } => self.click(x, y),
            InputEvent::SecondaryClick { x, y } => self.secondary_click(x, y),
            InputEvent::Press { x, y } => self.press_at(x, y),
            InputEvent::PointerMoved { x, y } => {
                let hover = hover_chrome(Point::new(x, y), self.tabs.len(), self.viewport_width);
                let popup_changed = self.update_popup_hover(Point::new(x, y));
                let hover_changed = hover != self.hover || popup_changed;
                self.hover = hover;
                let over_chrome_button = !matches!(
                    hover,
                    ChromeHover::Nothing | ChromeHover::Toolbar(ToolbarTarget::Address)
                );
                let link = if over_chrome_button {
                    None
                } else {
                    self.target_at(x, y)
                };
                let hovered_link = link.as_ref().map(LinkAction::hover_text);
                let link_changed = hovered_link != self.hovered_link;
                self.hovered_link = hovered_link;
                self.is_over_link = over_chrome_button
                    || link.is_some()
                    || self.popup_hover.is_some()
                    || self.start_hover
                    || self.is_over_clickable_control(x, y);
                if hover_changed || link_changed {
                    SceneReaction::Redraw
                } else {
                    SceneReaction::Idle
                }
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
        let mut wait = timer_delay(&mut self.tabs);
        if self.pending.is_some() {
            wait = Some(wait.map_or(LOAD_POLL, |delay| delay.min(LOAD_POLL)));
        }
        if let Some(notice) = &self.notice {
            let left = notice.remaining();
            wait = Some(wait.map_or(left, |delay| delay.min(left)));
        }
        wait
    }

    fn tick(&mut self) -> SceneReaction {
        let loaded = self.finish_load();
        let expired = self.notice.as_ref().is_some_and(Notice::is_expired);
        if expired {
            self.notice = None;
            self.pending_login = None;
        }
        run_due_timers(&mut self.tabs);
        let navigated = self.take_script_navigation().is_some();
        if refresh_active(&mut self.tabs)
            || loaded
            || expired
            || navigated
            || self.pending.is_some()
        {
            SceneReaction::Redraw
        } else {
            SceneReaction::Idle
        }
    }

    fn title(&self) -> Option<String> {
        if let Some(pending) = &self.pending {
            return Some(format!("Loading {} - firerust", pending.url));
        }
        let zoom = self.zoom();
        if zoom.is_default() {
            return Some(format!("{} - firerust", self.tabs.active().title()));
        }
        Some(format!(
            "{} - {}% - firerust",
            self.tabs.active().title(),
            zoom.percent()
        ))
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
        let factor = self.factor();
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
            factor,
        );
        if let Some(thumb) = scrollbar_thumb(self.scroll.offset(), metrics, self.viewport_width) {
            let placed = Rect::from_xywh(
                thumb.origin.x,
                thumb.origin.y + CHROME_HEIGHT,
                thumb.size.width,
                thumb.size.height,
            );
            fill_rounded_rect(
                canvas,
                placed,
                placed.size.width / 2.0,
                self.palette().scrollbar,
            );
        }
    }
}
