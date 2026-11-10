use std::sync::Arc;

use fr_raster::surface::canvas::Canvas;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::app::window_config::WindowConfig;
use crate::present::softbuffer_presenter::SoftbufferPresenter;
use crate::scene::render_scene::RenderScene;

pub struct AppHandler<S: RenderScene> {
    config: WindowConfig,
    scene: S,
    window: Option<Arc<Window>>,
    presenter: Option<SoftbufferPresenter>,
}

impl<S: RenderScene> AppHandler<S> {
    pub fn new(config: WindowConfig, scene: S) -> Self {
        Self {
            config,
            scene,
            window: None,
            presenter: None,
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
            _ => {}
        }
    }
}
