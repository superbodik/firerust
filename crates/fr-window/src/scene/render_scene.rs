use fr_raster::surface::canvas::Canvas;

use crate::input::input_event::InputEvent;
use crate::scene::scene_reaction::SceneReaction;

pub trait RenderScene {
    fn render(&mut self, canvas: &mut Canvas);

    fn handle(&mut self, _event: InputEvent) -> SceneReaction {
        SceneReaction::Idle
    }
}
