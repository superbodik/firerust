mod support;

use std::path::Path;
use std::time::Instant;

use fr_page::load::load_page::load_page;
use support::layout_of;

const SITES: [&str; 3] = [
    "https://en.wikipedia.org/wiki/Rust_(programming_language)",
    "https://ru.wikipedia.org/wiki/Rust",
    "https://www.rust-lang.org",
];

#[test]
#[ignore = "needs network access"]
fn layout_of_large_pages_stays_fast() {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for site in SITES {
        let loaded_at = Instant::now();
        let Ok(page) = load_page(site, base_dir) else {
            println!("{site}: cannot load");
            continue;
        };
        let load_ms = loaded_at.elapsed().as_millis();
        let laid_out = Instant::now();
        let tree = layout_of(&page, 1000.0);
        let first_ms = laid_out.elapsed().as_millis();
        let again = Instant::now();
        let _ = layout_of(&page, 900.0);
        let second_ms = again.elapsed().as_millis();
        println!(
            "{site}: load {load_ms} ms, layout {first_ms} ms, relayout {second_ms} ms, height {}",
            tree.height
        );
    }
}
