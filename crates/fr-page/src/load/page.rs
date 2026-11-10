use fr_geom::color::rgba::Rgba;
use url::Url;

use crate::lines::line::Line;

#[derive(Debug, Clone)]
pub struct Page {
    pub url: Url,
    pub title: Option<String>,
    pub background: Rgba,
    pub lines: Vec<Line>,
}
