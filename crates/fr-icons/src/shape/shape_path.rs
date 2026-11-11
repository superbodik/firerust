use fr_geom::vector::vector_path::VectorPath;

#[derive(Debug, Clone, PartialEq)]
pub struct ShapePath {
    pub path: VectorPath,
    pub even_odd: bool,
}
