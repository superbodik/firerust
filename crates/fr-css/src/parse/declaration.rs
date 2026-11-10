use fr_geom::color::rgba::Rgba;

use crate::value::display::Display;
use crate::value::length::Length;
use crate::value::side::Side;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Color(Rgba),
    BackgroundColor(Rgba),
    FontSize(Length),
    FontWeight(u16),
    Display(Display),
    Margin(Side, Length),
    Padding(Side, Length),
    BorderWidth(Side, Length),
    BorderColor(Rgba),
}
