use std::sync::Arc;

use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use fr_image::decode::decoded_image::DecodedImage;
use fr_layout::boxes::layout_run::LayoutRun;
use fr_text::render::text_alignment::TextAlignment;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayItem {
    Rect {
        rect: Rect,
        color: Rgba,
    },
    Text {
        rect: Rect,
        alignment: TextAlignment,
        runs: Vec<LayoutRun>,
    },
    Image {
        rect: Rect,
        image: Arc<DecodedImage>,
    },
}

impl DisplayItem {
    pub fn rect(&self) -> Rect {
        match self {
            Self::Rect { rect, .. } | Self::Text { rect, .. } | Self::Image { rect, .. } => *rect,
        }
    }
}
