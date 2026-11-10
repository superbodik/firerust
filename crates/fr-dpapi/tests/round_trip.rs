#![cfg(windows)]

use fr_dpapi::protect::protect;
use fr_dpapi::unprotect::unprotect;

const ENTROPY: &[u8] = b"test-entropy";

#[test]
fn protected_data_comes_back_unchanged() {
    let secret = "пароль: hunter2 \u{1F512}".as_bytes();

    let sealed = protect(secret, ENTROPY).expect("protected");
    let opened = unprotect(&sealed, ENTROPY).expect("unprotected");

    assert_eq!(opened, secret);
}

#[test]
fn protected_data_does_not_contain_the_plain_text() {
    let secret = b"hunter2-hunter2-hunter2";

    let sealed = protect(secret, ENTROPY).expect("protected");

    assert!(!sealed.windows(secret.len()).any(|part| part == secret));
}

#[test]
fn the_wrong_entropy_cannot_open_the_data() {
    let sealed = protect(b"secret", ENTROPY).expect("protected");

    assert!(unprotect(&sealed, b"other-entropy").is_none());
}

#[test]
fn garbage_cannot_be_opened() {
    assert!(unprotect(b"this was never protected", ENTROPY).is_none());
}

#[test]
fn empty_data_round_trips() {
    let sealed = protect(b"", ENTROPY).expect("protected");

    assert_eq!(unprotect(&sealed, ENTROPY).expect("unprotected"), b"");
}
