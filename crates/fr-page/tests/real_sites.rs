use std::path::Path;

use fr_page::load::load_page::load_page;

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
                println!(
                    "{site}: title={:?} lines={} background={:?}",
                    page.title,
                    page.lines.len(),
                    page.background
                );
                if page.title.is_none() || page.lines.is_empty() {
                    failures.push(format!("{site}: empty page"));
                }
            }
            Err(error) => failures.push(format!("{site}: {error}")),
        }
    }

    assert!(failures.is_empty(), "{failures:?}");
}
