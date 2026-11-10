use fr_mem::meter::process_memory::ProcessMemory;
use fr_page::load::build_page::build_page;
use fr_tab::set::tab_set::TabSet;
use fr_tab::summary::memory_summary::memory_summary;
use url::Url;

fn page(name: &str) -> fr_page::load::page::Page {
    let url = Url::parse(&format!("https://example.com/{name}")).unwrap();
    build_page("<p>text</p>", url)
}

#[test]
fn the_summary_names_the_process_memory() {
    let set = TabSet::new(page("a"));
    let process = ProcessMemory {
        resident_bytes: 31 * 1024 * 1024,
        virtual_bytes: 0,
    };

    let text = memory_summary(Some(process), set.tabs(), 0);

    assert!(text.starts_with("RAM 31.0 MB"));
}

#[test]
fn an_unreadable_process_is_reported_as_unknown() {
    let set = TabSet::new(page("a"));

    assert!(memory_summary(None, set.tabs(), 0).contains("RAM unknown"));
}

#[test]
fn discarded_tabs_are_marked() {
    let mut set = TabSet::new(page("a"));
    set.open(page("b"));
    set.enforce_limit(1);

    let text = memory_summary(None, set.tabs(), 0);

    assert!(text.contains("1: discarded"));
    assert!(!text.contains("2: discarded"));
}

#[test]
fn the_image_cache_size_is_included() {
    let set = TabSet::new(page("a"));

    let text = memory_summary(None, set.tabs(), 2 * 1024 * 1024);

    assert!(text.contains("image cache 2.0 MB"));
}
