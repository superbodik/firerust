use fr_passwords::model::change_kind::ChangeKind;
use fr_passwords::model::credential::Credential;
use fr_passwords::model::origin_of::origin_of;
use fr_passwords::model::password_list::PasswordList;

fn login(origin: &str, user: &str, password: &str) -> Credential {
    Credential::new(origin, user, password)
}

#[test]
fn a_new_login_is_added_and_the_same_one_changes_nothing() {
    let mut list = PasswordList::new();

    assert_eq!(
        list.save(login("https://a.test", "ada", "one")),
        ChangeKind::Added
    );
    assert_eq!(
        list.save(login("https://a.test", "ada", "one")),
        ChangeKind::Unchanged
    );
    assert_eq!(list.len(), 1);
}

#[test]
fn a_new_password_for_the_same_user_updates_the_login() {
    let mut list = PasswordList::new();
    list.save(login("https://a.test", "ada", "one"));

    assert_eq!(
        list.save(login("https://a.test", "ada", "two")),
        ChangeKind::Updated
    );
    assert_eq!(list.items()[0].password, "two");
    assert_eq!(list.len(), 1);
}

#[test]
fn the_same_user_on_another_site_is_a_separate_login() {
    let mut list = PasswordList::new();
    list.save(login("https://a.test", "ada", "one"));
    list.save(login("https://b.test", "ada", "one"));

    assert_eq!(list.len(), 2);
    assert_eq!(list.matching("https://b.test").len(), 1);
}

#[test]
fn logins_without_a_password_or_site_are_ignored() {
    let mut list = PasswordList::new();

    assert_eq!(
        list.save(login("https://a.test", "ada", "")),
        ChangeKind::Unchanged
    );
    assert_eq!(list.save(login("", "ada", "x")), ChangeKind::Unchanged);
    assert!(list.is_empty());
}

#[test]
fn removing_works_by_position() {
    let mut list = PasswordList::new();
    list.save(login("https://a.test", "ada", "one"));
    list.save(login("https://b.test", "bob", "two"));

    assert!(list.remove(0));
    assert!(!list.remove(5));
    assert_eq!(list.items()[0].username, "bob");
}

#[test]
fn the_debug_text_never_shows_the_password() {
    let text = format!("{:?}", login("https://a.test", "ada", "hunter2"));

    assert!(!text.contains("hunter2"));
    assert!(text.contains("ada"));
}

#[test]
fn origins_keep_scheme_host_and_port_only() {
    assert_eq!(
        origin_of("https://a.test/login?x=1"),
        Some("https://a.test".to_owned())
    );
    assert_eq!(
        origin_of("http://127.0.0.1:8080/account"),
        Some("http://127.0.0.1:8080".to_owned())
    );
    assert_eq!(origin_of("file:///C:/a.html"), None);
    assert_eq!(origin_of("javascript:alert(1)"), None);
    assert_eq!(origin_of("android://abc@com.app"), None);
    assert_eq!(origin_of("not an address"), None);
}
