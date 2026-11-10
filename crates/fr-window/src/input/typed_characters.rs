use winit::keyboard::ModifiersState;

pub fn typed_characters(text: Option<&str>, modifiers: ModifiersState) -> Vec<char> {
    if modifiers.control_key() || modifiers.alt_key() {
        return Vec::new();
    }
    text.unwrap_or_default()
        .chars()
        .filter(|character| !character.is_control())
        .collect()
}
