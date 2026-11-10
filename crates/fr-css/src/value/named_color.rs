use fr_geom::color::rgba::Rgba;

pub fn named_color(name: &str) -> Option<Rgba> {
    let color = match name.to_ascii_lowercase().as_str() {
        "black" => Rgba::opaque(0, 0, 0),
        "white" => Rgba::opaque(255, 255, 255),
        "red" => Rgba::opaque(255, 0, 0),
        "green" => Rgba::opaque(0, 128, 0),
        "blue" => Rgba::opaque(0, 0, 255),
        "yellow" => Rgba::opaque(255, 255, 0),
        "gray" | "grey" => Rgba::opaque(128, 128, 128),
        "silver" => Rgba::opaque(192, 192, 192),
        "maroon" => Rgba::opaque(128, 0, 0),
        "olive" => Rgba::opaque(128, 128, 0),
        "lime" => Rgba::opaque(0, 255, 0),
        "aqua" | "cyan" => Rgba::opaque(0, 255, 255),
        "teal" => Rgba::opaque(0, 128, 128),
        "navy" => Rgba::opaque(0, 0, 128),
        "fuchsia" | "magenta" => Rgba::opaque(255, 0, 255),
        "purple" => Rgba::opaque(128, 0, 128),
        "orange" => Rgba::opaque(255, 165, 0),
        "brown" => Rgba::opaque(165, 42, 42),
        "pink" => Rgba::opaque(255, 192, 203),
        "transparent" => Rgba::new(0, 0, 0, 0),
        _ => return None,
    };
    Some(color)
}
