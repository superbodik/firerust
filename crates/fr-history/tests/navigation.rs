use fr_history::history::navigation_history::NavigationHistory;
use url::Url;

fn url(path: &str) -> Url {
    Url::parse(&format!("https://example.com/{path}")).unwrap()
}

fn current_path(history: &NavigationHistory) -> String {
    history.current().unwrap().url.path().to_owned()
}

#[test]
fn a_new_history_is_empty() {
    let history = NavigationHistory::new();

    assert!(history.is_empty());
    assert!(history.current().is_none());
    assert!(!history.can_go_back());
    assert!(!history.can_go_forward());
}

#[test]
fn the_first_visit_becomes_the_current_entry() {
    let mut history = NavigationHistory::new();

    history.visit(url("a"));

    assert_eq!(current_path(&history), "/a");
    assert_eq!(history.len(), 1);
    assert!(!history.can_go_back());
}

#[test]
fn visiting_pages_builds_a_back_stack() {
    let mut history = NavigationHistory::new();
    history.visit(url("a"));
    history.visit(url("b"));
    history.visit(url("c"));

    assert_eq!(history.go_back().unwrap().url.path(), "/b");
    assert_eq!(history.go_back().unwrap().url.path(), "/a");
    assert!(history.go_back().is_none());
    assert_eq!(current_path(&history), "/a");
}

#[test]
fn going_forward_undoes_going_back() {
    let mut history = NavigationHistory::new();
    history.visit(url("a"));
    history.visit(url("b"));
    history.go_back();

    assert!(history.can_go_forward());
    assert_eq!(history.go_forward().unwrap().url.path(), "/b");
    assert!(history.go_forward().is_none());
}

#[test]
fn visiting_after_going_back_drops_the_forward_entries() {
    let mut history = NavigationHistory::new();
    history.visit(url("a"));
    history.visit(url("b"));
    history.go_back();

    history.visit(url("c"));

    assert_eq!(history.len(), 2);
    assert!(!history.can_go_forward());
    assert_eq!(history.go_back().unwrap().url.path(), "/a");
}

#[test]
fn the_scroll_position_is_kept_per_entry() {
    let mut history = NavigationHistory::new();
    history.visit(url("a"));
    history.remember_scroll(340.0);
    history.visit(url("b"));
    history.remember_scroll(90.0);

    assert_eq!(history.go_back().unwrap().scroll, 340.0);
    assert_eq!(history.go_forward().unwrap().scroll, 90.0);
}

#[test]
fn a_fresh_entry_starts_at_the_top() {
    let mut history = NavigationHistory::new();
    history.visit(url("a"));

    assert_eq!(history.current().unwrap().scroll, 0.0);
}
