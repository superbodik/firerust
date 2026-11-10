#![allow(dead_code)]

use std::sync::Arc;

use fr_css::cascade::compute_styles::compute_styles;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_dom::tree::edge::Edge;
use fr_html::parse::parse_html::parse_html;
use fr_image::decode::decoded_image::DecodedImage;
use fr_image::store::image_store::ImageStore;
use fr_layout::page::layout_page::layout_page;
use fr_paint::build::build_display_list::build_display_list;
use fr_paint::display::display_list::DisplayList;
use fr_paint::paint::paint_display_list::paint_display_list;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;

pub const WIDTH: u32 = 200;
pub const HEIGHT: u32 = 120;

pub struct Painted {
    pub canvas: Canvas,
    pub list: DisplayList,
}

impl Painted {
    pub fn rgb(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let pixel = self.canvas.pixmap().pixel(x, y).expect("pixel in range");
        (pixel.red(), pixel.green(), pixel.blue())
    }
}

pub fn solid_image(width: u32, height: u32, rgba: [u8; 4]) -> Arc<DecodedImage> {
    let data: Vec<u8> = (0..width * height).flat_map(|_| rgba).collect();
    Arc::new(DecodedImage::new(width, height, data))
}

pub fn paint(html: &str, css: &str, scroll_y: f32) -> Painted {
    paint_with_image(html, css, scroll_y, None)
}

pub fn paint_with_image(
    html: &str,
    css: &str,
    scroll_y: f32,
    image: Option<Arc<DecodedImage>>,
) -> Painted {
    let document = parse_html(html);
    let author = [fr_css::parse::parse_stylesheet::parse_stylesheet(css)];
    let styles = compute_styles(&document, &user_agent_sheet(), &author);
    let mut images = ImageStore::new();
    if let Some(picture) = image {
        let img_id = document
            .walk(document.root())
            .find_map(|edge| match edge {
                Edge::Enter(id) => {
                    let element = document.node(id)?.kind().as_element()?;
                    (element.name() == "img").then_some(id)
                }
                Edge::Leave(_) => None,
            })
            .expect("img element");
        images.insert(img_id, picture);
    }
    let mut engine = TextEngine::new();
    let tree = layout_page(&document, &styles, &images, &mut engine, WIDTH as f32);
    let list = build_display_list(&tree.root);
    let mut canvas = Canvas::new(WIDTH, HEIGHT).expect("canvas");
    fr_raster::draw::clear::clear(&mut canvas, tree.background);
    paint_display_list(&mut canvas, &mut engine, &list, scroll_y);
    Painted { canvas, list }
}
