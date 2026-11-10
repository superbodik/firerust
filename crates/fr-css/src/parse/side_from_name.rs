use crate::value::side::Side;

pub fn side_from_name(name: &str) -> Option<Side> {
    match name {
        "top" => Some(Side::Top),
        "right" => Some(Side::Right),
        "bottom" => Some(Side::Bottom),
        "left" => Some(Side::Left),
        _ => None,
    }
}
