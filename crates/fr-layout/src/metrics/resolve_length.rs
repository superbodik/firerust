use fr_css::value::length::Length;

pub fn resolve_length(length: Length, base: f32) -> Option<f32> {
    match length {
        Length::Px(value) => Some(value),
        Length::Percent(percent) => Some(base * percent / 100.0),
        Length::Em(_) | Length::Rem(_) | Length::Auto => None,
    }
}
