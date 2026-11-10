use fr_page::load::build_page::build_page;
use fr_page::load::page::Page;
use fr_tab::set::tab_set::TabSet;
use url::Url;

fn page(name: &str) -> Page {
    let url = Url::parse(&format!("https://example.com/{name}")).unwrap();
    build_page(
        &format!("<html><head><title>{name}</title></head><body><p>{name}</p></body></html>"),
        url,
    )
}

fn set_of(names: &[&str]) -> TabSet {
    let mut set = TabSet::new(page(names[0]));
    for name in &names[1..] {
        set.open(page(name));
    }
    set
}

#[test]
fn a_new_set_has_one_active_tab() {
    let set = TabSet::new(page("a"));

    assert_eq!(set.len(), 1);
    assert_eq!(set.active_index(), 0);
    assert_eq!(set.active().title(), "a");
}

#[test]
fn opening_a_tab_makes_it_active() {
    let mut set = TabSet::new(page("a"));

    let index = set.open(page("b"));

    assert_eq!(index, 1);
    assert_eq!(set.active_index(), 1);
    assert_eq!(set.active().title(), "b");
}

#[test]
fn every_tab_gets_its_own_id() {
    let set = set_of(&["a", "b", "c"]);

    let ids: Vec<_> = set.tabs().iter().map(|tab| tab.id()).collect();
    assert_ne!(ids[0], ids[1]);
    assert_ne!(ids[1], ids[2]);
    assert_ne!(ids[0], ids[2]);
}

#[test]
fn activating_switches_the_active_tab() {
    let mut set = set_of(&["a", "b", "c"]);

    assert!(set.activate(0));

    assert_eq!(set.active().title(), "a");
    assert!(!set.activate(0));
    assert!(!set.activate(9));
}

#[test]
fn closing_the_last_tab_is_refused() {
    let mut set = TabSet::new(page("a"));

    assert!(!set.close(0));
    assert_eq!(set.len(), 1);
}

#[test]
fn closing_a_tab_before_the_active_one_keeps_the_same_tab_active() {
    let mut set = set_of(&["a", "b", "c"]);

    set.close(0);

    assert_eq!(set.active().title(), "c");
    assert_eq!(set.active_index(), 1);
}

#[test]
fn closing_the_active_tab_activates_its_neighbour() {
    let mut set = set_of(&["a", "b", "c"]);
    set.activate(1);

    set.close(1);

    assert_eq!(set.active().title(), "c");
}

#[test]
fn closing_the_active_last_tab_activates_the_previous_one() {
    let mut set = set_of(&["a", "b", "c"]);

    set.close(2);

    assert_eq!(set.active().title(), "b");
}

#[test]
fn closing_a_tab_after_the_active_one_changes_nothing_for_it() {
    let mut set = set_of(&["a", "b", "c"]);
    set.activate(0);

    set.close(2);

    assert_eq!(set.active().title(), "a");
    assert_eq!(set.len(), 2);
}

#[test]
fn the_tab_limit_discards_the_least_recently_used_background_tabs() {
    let mut set = set_of(&["a", "b", "c", "d"]);
    set.activate(0);
    set.activate(1);
    set.activate(2);

    let discarded = set.enforce_limit(2);

    assert_eq!(discarded, 2);
    assert!(!set.tabs()[2].is_discarded());
    assert!(!set.tabs()[1].is_discarded());
    assert!(set.tabs()[0].is_discarded());
    assert!(set.tabs()[3].is_discarded());
}

#[test]
fn the_active_tab_is_never_discarded() {
    let mut set = set_of(&["a", "b", "c"]);

    set.enforce_limit(1);

    assert!(!set.active().is_discarded());
    assert_eq!(
        set.tabs().iter().filter(|tab| !tab.is_discarded()).count(),
        1
    );
}

#[test]
fn a_roomy_limit_discards_nothing() {
    let mut set = set_of(&["a", "b", "c"]);

    assert_eq!(set.enforce_limit(5), 0);
}

#[test]
fn a_discarded_tab_keeps_its_address_title_history_and_scroll() {
    let mut set = set_of(&["a", "b"]);
    set.tab_mut(0).unwrap().set_scroll(120.0);

    set.enforce_limit(1);

    let tab = &set.tabs()[0];
    assert!(tab.is_discarded());
    assert_eq!(tab.url().path(), "/a");
    assert_eq!(tab.title(), "a");
    assert_eq!(tab.scroll(), 120.0);
    assert_eq!(tab.history().len(), 1);
}

#[test]
fn a_discarded_tab_can_be_restored_with_a_fresh_page() {
    let mut set = set_of(&["a", "b"]);
    set.enforce_limit(1);

    set.tab_mut(0).unwrap().restore(page("a"));

    assert!(!set.tabs()[0].is_discarded());
}

#[test]
fn a_tab_that_is_out_of_range_cannot_be_borrowed() {
    let mut set = TabSet::new(page("a"));

    assert!(set.tab_mut(3).is_none());
}
