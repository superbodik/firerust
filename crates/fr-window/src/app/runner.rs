use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

use crate::app::app_handler::AppHandler;
use crate::app::window_config::WindowConfig;
use crate::scene::render_scene::RenderScene;

pub fn run<S: RenderScene>(config: WindowConfig, scene: S) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut AppHandler::new(config, scene))
}
