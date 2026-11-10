use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::image::attr_length::attr_length;

pub fn image_widths(
    context: &LayoutContext<'_>,
    id: NodeId,
    style: &ComputedStyle,
) -> ContentWidths {
    let natural = context
        .images()
        .get(id)
        .map_or(0.0, |image| image.width() as f32);
    let width = match style.size.width {
        Length::Px(value) => value,
        _ => match attr_length(context.document(), id, "width") {
            Some(Length::Px(value)) => value,
            _ => natural,
        },
    };
    ContentWidths {
        min: width,
        max: width,
    }
}
