use std::num::NonZeroU32;
use std::sync::Arc;

use fr_raster::export::xrgb::copy_into_xrgb;
use fr_raster::surface::canvas::Canvas;
use winit::window::Window;

pub struct SoftbufferPresenter {
    _context: softbuffer::Context<Arc<Window>>,
    surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
}

impl SoftbufferPresenter {
    pub fn new(window: Arc<Window>) -> Result<Self, softbuffer::SoftBufferError> {
        let context = softbuffer::Context::new(window.clone())?;
        let surface = softbuffer::Surface::new(&context, window)?;
        Ok(Self {
            _context: context,
            surface,
        })
    }

    pub fn present(&mut self, canvas: &Canvas) -> Result<(), softbuffer::SoftBufferError> {
        let (Some(width), Some(height)) = (
            NonZeroU32::new(canvas.width()),
            NonZeroU32::new(canvas.height()),
        ) else {
            return Ok(());
        };
        self.surface.resize(width, height)?;
        let mut buffer = self.surface.buffer_mut()?;
        copy_into_xrgb(canvas.pixmap(), &mut buffer);
        buffer.present()
    }
}
