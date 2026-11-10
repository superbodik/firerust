use std::error::Error;

use fr_page::load::blank_page::blank_page;
use fr_page::load::load_page::load_page;
use fr_page::load::page::Page;
use fr_window::app::runner::run;
use fr_window::app::window_config::WindowConfig;

use crate::browser::window_title::window_title;
use crate::scenes::page_scene::PageScene;

pub fn run_browser() -> Result<(), Box<dyn Error>> {
    let page = first_page()?;
    let config = WindowConfig::new(window_title(&page), 960, 600);
    run(config, PageScene::new(page))?;
    Ok(())
}

fn first_page() -> Result<Page, Box<dyn Error>> {
    match std::env::args().nth(1) {
        Some(address) => {
            let base_dir = std::env::current_dir()?;
            Ok(load_page(&address, &base_dir)?)
        }
        None => blank_page().ok_or_else(|| "cannot build the start page".into()),
    }
}
