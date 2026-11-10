use std::sync::Arc;
use std::time::Instant;

use fr_raster::surface::canvas::Canvas;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::ModifiersState;
use winit::window::{CursorIcon, Window, WindowId};

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
        window.set_cursor(match self.scene.pointer() {
            PointerKind::Arrow => CursorIcon::Default,
            PointerKind::Hand => CursorIcon::Pointer,
        });
        if reaction == SceneReaction::Redraw {
            window.request_redraw();
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
        let Some(mut canvas) = Canvas::new(size.width, size.height) else {
            return;
        };
        self.scene.render(&mut canvas);
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
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height));
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
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => self.dispatch(InputEvent::Wheel {
                delta_y: translate_wheel(delta),
            }),
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
                self.cursor = (position.x as f32, position.y as f32);
                let (x, y) = self.cursor;
                self.dispatch(InputEvent::PointerMoved { x, y });
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
