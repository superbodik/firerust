use fr_raster::surface::canvas::Canvas;

pub trait RenderScene {
    fn render(&mut self, canvas: &mut Canvas);
}
