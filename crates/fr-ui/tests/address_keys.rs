use fr_ui::address::address_field::AddressField;
use fr_ui::address::address_outcome::AddressOutcome;
use fr_ui::address::apply_address_key::apply_address_key;
use fr_window::input::key_input::KeyInput;

fn field(text: &str) -> AddressField {
    let mut field = AddressField::new();
    field.set_text(text);
    field
}

#[test]
fn enter_submits() {
    let mut field = field("example.com");

    assert_eq!(
        apply_address_key(&mut field, KeyInput::Enter),
        AddressOutcome::Submit
    );
    assert_eq!(field.text(), "example.com");
}

#[test]
fn escape_cancels() {
    let mut field = field("example.com");

    assert_eq!(
        apply_address_key(&mut field, KeyInput::Escape),
        AddressOutcome::Cancel
    );
}

#[test]
fn backspace_edits_the_text() {
    let mut field = field("ab");

    assert_eq!(
        apply_address_key(&mut field, KeyInput::Backspace),
        AddressOutcome::Edited
    );
    assert_eq!(field.text(), "a");
}

#[test]
fn space_types_a_space() {
    let mut field = field("a");

    apply_address_key(&mut field, KeyInput::Space);

    assert_eq!(field.text(), "a ");
}

#[test]
fn caret_keys_move_the_caret() {
    let mut field = field("abc");

    apply_address_key(&mut field, KeyInput::Home);
    assert_eq!(field.caret(), 0);
    apply_address_key(&mut field, KeyInput::ArrowRight);
    assert_eq!(field.caret(), 1);
    apply_address_key(&mut field, KeyInput::ArrowLeft);
    assert_eq!(field.caret(), 0);
    apply_address_key(&mut field, KeyInput::End);
    assert_eq!(field.caret(), 3);
}

#[test]
fn delete_removes_after_the_caret() {
    let mut field = field("abc");
    apply_address_key(&mut field, KeyInput::Home);

    apply_address_key(&mut field, KeyInput::Delete);

    assert_eq!(field.text(), "bc");
}

#[test]
fn page_scrolling_keys_are_left_alone() {
    let mut field = field("abc");

    for key in [KeyInput::ArrowUp, KeyInput::ArrowDown, KeyInput::PageDown] {
        assert_eq!(apply_address_key(&mut field, key), AddressOutcome::Ignored);
    }
    assert_eq!(field.text(), "abc");
}
