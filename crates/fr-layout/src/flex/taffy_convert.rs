use fr_css::value::length::Length;
use taffy::prelude::{
    Dimension, LengthPercentage, LengthPercentageAuto, auto, length, percent, zero,
};

pub fn dimension(value: Length) -> Dimension {
    match value {
        Length::Px(pixels) => length(pixels),
        Length::Percent(share) => percent(share / 100.0),
        Length::Em(_) | Length::Rem(_) | Length::Auto => auto(),
    }
}

pub fn length_percentage_auto(value: Length) -> LengthPercentageAuto {
    match value {
        Length::Px(pixels) => length(pixels),
        Length::Percent(share) => percent(share / 100.0),
        Length::Em(_) | Length::Rem(_) | Length::Auto => auto(),
    }
}

pub fn length_percentage(value: Length) -> LengthPercentage {
    match value {
        Length::Px(pixels) => length(pixels),
        Length::Percent(share) => percent(share / 100.0),
        Length::Em(_) | Length::Rem(_) | Length::Auto => zero(),
    }
}
