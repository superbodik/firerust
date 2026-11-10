use fr_raster::surface::canvas::Canvas;

use crate::input::input_event::InputEvent;
use crate::scene::pointer_kind::PointerKind;
use crate::scene::scene_reaction::SceneReaction;

pub trait RenderScene {
    fn render(&mut self, canvas: &mut Canvas);

    fn handle(&mut self, _event: InputEvent) -> SceneReaction {
        SceneReaction::Idle
    }

    fn title(&self) -> Option<String> {
        None
    }

    fn pointer(&self) -> PointerKind {
        PointerKind::Arrow
    }
}
