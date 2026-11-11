use std::error::Error;

use fr_page::load::blank_page::blank_page;
use fr_page::load::load_page::load_page;
use fr_window::app::runner::run;
use fr_window::app::window_config::WindowConfig;

use crate::browser::window_icon_pixels::window_icon_pixels;
use crate::browser::window_title::window_title;
use crate::scenes::page_scene::PageScene;

pub fn run_browser() -> Result<(), Box<dyn Error>> {
    let addresses: Vec<String> = std::env::args().skip(1).collect();
    let first = blank_page().ok_or("cannot build the start page")?;
    let config = WindowConfig::new(window_title(&first), 960, 600).with_icon(window_icon_pixels());
    let mut scene = PageScene::new(first);
    let base_dir = std::env::current_dir()?;
    for address in addresses.iter().skip(1) {
        scene.add_background_tab(load_page(address, &base_dir)?);
    }
    if let Some(address) = addresses.first() {
        scene.open_first(address);
    }
    run(config, scene)?;
    Ok(())
}
