use std::sync::Arc;
use std::time::Instant;

use fr_geom::color::rgba::Rgba;
use fr_raster::draw::blit_canvas::blit_canvas;
use fr_raster::draw::clear::clear;
use fr_raster::surface::canvas::Canvas;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::ModifiersState;
use winit::platform::windows::WindowAttributesExtWindows;
use winit::window::{CursorIcon, ResizeDirection, Theme, Window, WindowId};

use crate::app::resize_edge::{cursor_for_edge, resize_edge};
use crate::app::window_config::WindowConfig;
use crate::input::input_event::InputEvent;
use crate::input::key_input::KeyInput;
use crate::input::translate_key::translate_key;
use crate::input::translate_wheel::translate_wheel;
use crate::input::typed_characters::typed_characters;
use crate::present::softbuffer_presenter::SoftbufferPresenter;
use crate::scene::pointer_kind::PointerKind;
use crate::scene::render_scene::RenderScene;
use crate::scene::scene_reaction::SceneReaction;

pub struct AppHandler<S: RenderScene> {
    config: WindowConfig,
    scene: S,
    window: Option<Arc<Window>>,
    presenter: Option<SoftbufferPresenter>,
    cursor: (f32, f32),
    modifiers: ModifiersState,
    shown_title: String,
    edge: Option<ResizeDirection>,
    should_exit: bool,
}

fn maximized_inset(window: &Window) -> (u32, u32) {
    if !window.is_maximized() {
        return (0, 0);
    }
    match window.outer_position() {
        Ok(position) => (
            position.x.min(0).unsigned_abs(),
            position.y.min(0).unsigned_abs(),
        ),
        Err(_) => (0, 0),
    }
}

impl<S: RenderScene> AppHandler<S> {
    pub fn new(config: WindowConfig, scene: S) -> Self {
        Self {
            shown_title: config.title.clone(),
            config,
            scene,
            window: None,
            presenter: None,
            cursor: (0.0, 0.0),
            modifiers: ModifiersState::empty(),
            edge: None,
            should_exit: false,
        }
    }

    fn dispatch(&mut self, event: InputEvent) {
        let reaction = self.scene.handle(event);
        self.apply(reaction);
    }

    fn apply(&mut self, reaction: SceneReaction) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if let Some(title) = self.scene.title()
            && title != self.shown_title
        {
            window.set_title(&title);
            self.shown_title = title;
        }
        let cursor = match self.edge {
            Some(direction) => cursor_for_edge(direction),
            None => match self.scene.pointer() {
                PointerKind::Arrow => CursorIcon::Default,
                PointerKind::Hand => CursorIcon::Pointer,
            },
        };
        window.set_cursor(cursor);
        match reaction {
            SceneReaction::Idle => {}
            SceneReaction::Redraw => window.request_redraw(),
            SceneReaction::DragWindow => {
                if let Err(error) = window.drag_window() {
                    eprintln!("cannot move the window: {error}");
                }
            }
            SceneReaction::Minimize => window.set_minimized(true),
            SceneReaction::ToggleMaximize => window.set_maximized(!window.is_maximized()),
            SceneReaction::CloseWindow => self.should_exit = true,
        }
    }

    fn redraw(&mut self) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        let Some(presenter) = self.presenter.as_mut() else {
            return;
        };
        let size = window.inner_size();
        let (inset_x, inset_y) = maximized_inset(window);
        let Some(mut canvas) = Canvas::new(size.width, size.height) else {
            return;
        };
        if inset_x == 0 && inset_y == 0 {
            self.scene.render(&mut canvas);
        } else {
            let inner_width = size.width.saturating_sub(2 * inset_x);
            let inner_height = size.height.saturating_sub(2 * inset_y);
            if let Some(mut inner) = Canvas::new(inner_width, inner_height) {
                self.scene.render(&mut inner);
                clear(&mut canvas, Rgba::BLACK);
                blit_canvas(&mut canvas, &inner, inset_x as i32, inset_y as i32);
            }
        }
        if let Err(error) = presenter.present(&canvas) {
            eprintln!("present failed: {error}");
        }
    }
}

impl<S: RenderScene> ApplicationHandler for AppHandler<S> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if matches!(cause, StartCause::ResumeTimeReached { .. }) {
            let reaction = self.scene.tick();
            self.apply(reaction);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.should_exit {
            event_loop.exit();
            return;
        }
        match self.scene.next_wakeup() {
            Some(delay) => {
                event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + delay))
            }
            None => event_loop.set_control_flow(ControlFlow::Wait),
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title(self.config.title.clone())
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
            .with_decorations(false)
            .with_undecorated_shadow(true);
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                eprintln!("window creation failed: {error}");
                event_loop.exit();
                return;
            }
        };
        match SoftbufferPresenter::new(window.clone()) {
            Ok(presenter) => self.presenter = Some(presenter),
            Err(error) => {
                eprintln!("presenter creation failed: {error}");
                event_loop.exit();
                return;
            }
        }
        let dark = window.theme() == Some(Theme::Dark);
        self.scene.set_dark(dark);
        self.scene.set_maximized(window.is_maximized());
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.redraw(),
            WindowEvent::Resized(_) => {
                let maximized = self
                    .window
                    .as_ref()
                    .is_some_and(|window| window.is_maximized());
                let reaction = self.scene.set_maximized(maximized);
                self.apply(reaction);
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let delta_y = translate_wheel(delta);
                if self.modifiers.control_key() {
                    let key = if delta_y < 0.0 {
                        KeyInput::ZoomIn
                    } else {
                        KeyInput::ZoomOut
                    };
                    if delta_y != 0.0 {
                        self.dispatch(InputEvent::Key(key));
                    }
                } else {
                    self.dispatch(InputEvent::Wheel { delta_y });
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }
                match translate_key(&event.logical_key, event.physical_key, self.modifiers) {
                    Some(key) => self.dispatch(InputEvent::Key(key)),
                    None => {
                        let text = event.text.as_deref();
                        for character in typed_characters(text, self.modifiers) {
                            self.dispatch(InputEvent::Character(character));
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let (inset_x, inset_y) = self
                    .window
                    .as_ref()
                    .map_or((0, 0), |window| maximized_inset(window));
                self.cursor = (
                    position.x as f32 - inset_x as f32,
                    position.y as f32 - inset_y as f32,
                );
                let (x, y) = self.cursor;
                self.edge = self.window.as_ref().and_then(|window| {
                    if window.is_maximized() {
                        return None;
                    }
                    let size = window.inner_size();
                    resize_edge(x, y, size.width as f32, size.height as f32)
                });
                self.dispatch(InputEvent::PointerMoved { x, y });
            }
            WindowEvent::CursorLeft { .. } => {
                self.dispatch(InputEvent::PointerMoved { x: -1.0, y: -1.0 });
            }
            WindowEvent::ThemeChanged(theme) => {
                let reaction = self.scene.set_dark(theme == Theme::Dark);
                self.apply(reaction);
            }
            WindowEvent::ModifiersChanged(modifiers) => self.modifiers = modifiers.state(),
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Back,
                ..
            } => self.dispatch(InputEvent::Key(KeyInput::GoBack)),
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Forward,
                ..
            } => self.dispatch(InputEvent::Key(KeyInput::GoForward)),
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let (Some(direction), Some(window)) = (self.edge, self.window.as_ref()) {
                    if let Err(error) = window.drag_resize_window(direction) {
                        eprintln!("cannot resize the window: {error}");
                    }
                    return;
                }
                let (x, y) = self.cursor;
                self.dispatch(InputEvent::Press { x, y });
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                let (x, y) = self.cursor;
                self.dispatch(InputEvent::Click { x, y });
            }
            _ => {}
        }
    }
}
