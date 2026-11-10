#![cfg(windows)]

use std::path::PathBuf;

use fr_passwords::model::change_kind::ChangeKind;
use fr_passwords::model::credential::Credential;
use fr_passwords::storage::password_vault::{ImportCounts, PasswordVault};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("firerust-vault-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir.join("passwords.bin")
}

fn cleanup(path: &std::path::Path) {
    if let Some(dir) = path.parent() {
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[test]
fn saved_logins_are_found_again_after_reopening() {
    let path = scratch("reopen");
    {
        let mut vault = PasswordVault::at(Some(path.clone()));
        vault.save_login(Credential::new("https://a.test", "ada", "pass-one"));
        vault.save_login(Credential::new("https://b.test", "bob", "пароль"));
    }

    let reopened = PasswordVault::at(Some(path.clone()));

    assert_eq!(reopened.list().len(), 2);
    assert_eq!(reopened.matching("https://b.test")[0].password, "пароль");
    cleanup(&path);
}

#[test]
fn the_file_on_disk_does_not_contain_the_passwords() {
    let path = scratch("sealed");
    let mut vault = PasswordVault::at(Some(path.clone()));
    vault.save_login(Credential::new("https://a.test", "ada", "very-secret-text"));

    let bytes = std::fs::read(&path).expect("file");
    let text = String::from_utf8_lossy(&bytes);

    assert!(!text.contains("very-secret-text"));
    assert!(!text.contains("ada"));
    assert!(!text.contains("a.test"));
    cleanup(&path);
}

#[test]
fn a_damaged_file_opens_as_an_empty_vault() {
    let path = scratch("damaged");
    std::fs::create_dir_all(path.parent().expect("dir")).expect("create");
    std::fs::write(&path, b"garbage").expect("write");

    assert!(PasswordVault::at(Some(path.clone())).list().is_empty());
    cleanup(&path);
}

#[test]
fn removing_a_login_is_saved() {
    let path = scratch("remove");
    let mut vault = PasswordVault::at(Some(path.clone()));
    vault.save_login(Credential::new("https://a.test", "ada", "x"));
    vault.remove(0);

    assert!(PasswordVault::at(Some(path.clone())).list().is_empty());
    cleanup(&path);
}

#[test]
fn importing_counts_added_and_updated_logins() {
    let mut vault = PasswordVault::at(None);
    vault.save_login(Credential::new("https://a.test", "ada", "old"));

    let counts = vault.import(vec![
        Credential::new("https://a.test", "ada", "new"),
        Credential::new("https://b.test", "bob", "x"),
        Credential::new("https://b.test", "bob", "x"),
    ]);

    assert_eq!(
        counts,
        ImportCounts {
            found: 3,
            added: 1,
            updated: 1
        }
    );
}

#[test]
fn a_vault_without_a_file_still_works_in_memory() {
    let mut vault = PasswordVault::at(None);

    assert_eq!(
        vault.save_login(Credential::new("https://a.test", "ada", "x")),
        ChangeKind::Added
    );
    assert_eq!(vault.matching("https://a.test").len(), 1);
}
