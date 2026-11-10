use std::error::Error;

use fr_page::load::blank_page::blank_page;
use fr_page::load::load_page::load_page;
use fr_page::load::page::Page;
use fr_window::app::runner::run;
use fr_window::app::window_config::WindowConfig;

use crate::browser::window_title::window_title;
use crate::scenes::page_scene::PageScene;

pub fn run_browser() -> Result<(), Box<dyn Error>> {
    let mut pages = load_pages()?.into_iter();
    let first = pages.next().ok_or("no start page")?;
    let config = WindowConfig::new(window_title(&first), 960, 600);
    let mut scene = PageScene::new(first);
    for page in pages {
        scene.add_background_tab(page);
    }
    run(config, scene)?;
    Ok(())
}

fn load_pages() -> Result<Vec<Page>, Box<dyn Error>> {
    let addresses: Vec<String> = std::env::args().skip(1).collect();
    if addresses.is_empty() {
        let page = blank_page().ok_or("cannot build the start page")?;
        return Ok(vec![page]);
    }
    let base_dir = std::env::current_dir()?;
    let mut pages = Vec::new();
    for address in &addresses {
        pages.push(load_page(address, &base_dir)?);
    }
    Ok(pages)
}
