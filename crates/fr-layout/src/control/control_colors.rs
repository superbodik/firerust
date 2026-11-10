use fr_geom::color::rgba::Rgba;

use crate::control::control_kind::ControlKind;

pub const FOCUS_BORDER: Rgba = Rgba::opaque(11, 95, 165);
pub const IDLE_BORDER: Rgba = Rgba::opaque(118, 118, 118);
pub const PLACEHOLDER_TEXT: Rgba = Rgba::opaque(140, 140, 140);
pub const CONTROL_TEXT: Rgba = Rgba::opaque(20, 20, 20);

pub fn default_background(kind: ControlKind) -> Rgba {
    match kind {
        ControlKind::Button => Rgba::opaque(233, 233, 237),
        _ => Rgba::opaque(255, 255, 255),
    }
}
