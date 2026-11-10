use winit::event::MouseScrollDelta;

const LINE_HEIGHT_PIXELS: f32 = 48.0;

pub fn translate_wheel(delta: MouseScrollDelta) -> f32 {
    match delta {
        MouseScrollDelta::LineDelta(_, lines) => -lines * LINE_HEIGHT_PIXELS,
        MouseScrollDelta::PixelDelta(position) => -(position.y as f32),
    }
}
