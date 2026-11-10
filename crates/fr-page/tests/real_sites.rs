mod support;

use std::path::Path;

use fr_page::load::load_page::load_page;
use support::{layout_of, texts};

const SITES: [&str; 6] = [
    "https://example.com",
    "https://example.org",
    "http://info.cern.ch",
    "https://text.npr.org",
    "https://news.ycombinator.com",
    "https://lobste.rs",
];

#[test]
#[ignore = "needs network access"]
fn real_sites_give_a_title_and_text() {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut failures: Vec<String> = Vec::new();

    for site in SITES {
        match load_page(site, base_dir) {
            Ok(page) => {
                let tree = layout_of(&page, 1000.0);
                let text_count = texts(&tree).len();
                println!(
                    "{site}: title={:?} text boxes={text_count} images={} height={}",
                    page.title,
                    page.images.len(),
                    tree.height
                );
                if page.title.is_none() || text_count == 0 {
                    failures.push(format!("{site}: empty page"));
                }
            }
            Err(error) => failures.push(format!("{site}: {error}")),
        }
    }

    assert!(failures.is_empty(), "{failures:?}");
}
