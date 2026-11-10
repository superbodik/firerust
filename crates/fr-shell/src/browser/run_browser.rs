use std::error::Error;

use fr_page::load::load_page::load_page;
use fr_window::app::runner::run;
use fr_window::app::window_config::WindowConfig;

use crate::browser::window_title::window_title;
use crate::scenes::page_scene::PageScene;

pub fn run_browser() -> Result<(), Box<dyn Error>> {
    let address = std::env::args()
        .nth(1)
        .ok_or("usage: fr-shell <address or file>")?;
    let base_dir = std::env::current_dir()?;
    let page = load_page(&address, &base_dir)?;
    let config = WindowConfig::new(window_title(&page), 960, 600);
    run(config, PageScene::new(page.lines))?;
    Ok(())
}
