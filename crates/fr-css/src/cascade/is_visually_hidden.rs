use crate::value::length::Length;
use crate::value::position::Position;
use crate::value::size_style::SizeStyle;

const TINY: f32 = 2.0;

pub fn is_visually_hidden(position: Position, size: &SizeStyle) -> bool {
    if !position.is_out_of_flow() {
        return false;
    }
    let is_tiny = |length: Length| matches!(length, Length::Px(value) if value <= TINY);
    is_tiny(size.width) && is_tiny(size.height)
}
