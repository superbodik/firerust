use fr_ui::address::address_field::AddressField;

fn field(text: &str) -> AddressField {
    let mut field = AddressField::new();
    field.set_text(text);
    field
}

#[test]
fn a_new_field_is_empty_and_unfocused() {
    let field = AddressField::new();

    assert_eq!(field.text(), "");
    assert!(!field.is_focused());
}

#[test]
fn setting_text_puts_the_caret_at_the_end() {
    let field = field("abc");

    assert_eq!(field.caret(), 3);
    assert!(!field.selects_all());
}

#[test]
fn focusing_selects_everything() {
    let mut field = field("abc");

    field.focus();

    assert!(field.is_focused());
    assert!(field.selects_all());
}

#[test]
fn typing_over_a_selection_replaces_the_text() {
    let mut field = field("old address");
    field.focus();

    field.insert('n');
    field.insert('e');

    assert_eq!(field.text(), "ne");
    assert!(!field.selects_all());
}

#[test]
fn typing_inserts_at_the_caret() {
    let mut field = field("ac");
    field.move_left();

    field.insert('b');

    assert_eq!(field.text(), "abc");
    assert_eq!(field.caret(), 2);
}

#[test]
fn backspace_removes_the_character_before_the_caret() {
    let mut field = field("abc");

    field.backspace();

    assert_eq!(field.text(), "ab");
    assert_eq!(field.caret(), 2);
}

#[test]
fn backspace_at_the_start_does_nothing() {
    let mut field = field("abc");
    field.move_home();

    field.backspace();

    assert_eq!(field.text(), "abc");
    assert_eq!(field.caret(), 0);
}

#[test]
fn backspace_on_a_selection_clears_everything() {
    let mut field = field("abc");
    field.focus();

    field.backspace();

    assert_eq!(field.text(), "");
    assert_eq!(field.caret(), 0);
}

#[test]
fn delete_removes_the_character_after_the_caret() {
    let mut field = field("abc");
    field.move_home();

    field.delete();

    assert_eq!(field.text(), "bc");
    assert_eq!(field.caret(), 0);
}

#[test]
fn delete_at_the_end_does_nothing() {
    let mut field = field("abc");

    field.delete();

    assert_eq!(field.text(), "abc");
}

#[test]
fn the_caret_stays_inside_the_text() {
    let mut field = field("ab");

    field.move_right();
    assert_eq!(field.caret(), 2);
    field.move_home();
    field.move_left();
    assert_eq!(field.caret(), 0);
    field.move_end();
    assert_eq!(field.caret(), 2);
}

#[test]
fn moving_the_caret_drops_the_selection() {
    let mut field = field("abc");
    field.focus();

    field.move_left();
    field.insert('x');

    assert_eq!(field.text(), "abxc");
}

#[test]
fn non_ascii_characters_are_edited_whole() {
    let mut field = field("при");

    field.backspace();
    field.insert('в');
    field.move_home();
    field.delete();

    assert_eq!(field.text(), "рв");
}

#[test]
fn blurring_keeps_the_text() {
    let mut field = field("abc");
    field.focus();

    field.blur();

    assert!(!field.is_focused());
    assert_eq!(field.text(), "abc");
}
