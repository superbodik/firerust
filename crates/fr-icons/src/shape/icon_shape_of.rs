use std::sync::OnceLock;

use crate::catalog::icon::Icon;
use crate::parse::extract_paths::extract_paths;
use crate::parse::parse_svg_path::parse_svg_path;
use crate::shape::icon_shape::IconShape;
use crate::shape::shape_path::ShapePath;

fn build(icon: Icon) -> IconShape {
    let paths = extract_paths(icon.svg())
        .into_iter()
        .filter_map(|(data, even_odd)| {
            parse_svg_path(&data).map(|path| ShapePath { path, even_odd })
        })
        .collect();
    IconShape { paths }
}

pub fn icon_shape_of(icon: Icon) -> &'static IconShape {
    static SHAPES: OnceLock<Vec<IconShape>> = OnceLock::new();
    let shapes = SHAPES.get_or_init(|| Icon::ALL.iter().map(|icon| build(*icon)).collect());
    &shapes[icon.index()]
}
