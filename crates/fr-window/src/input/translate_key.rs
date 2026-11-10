use winit::keyboard::{Key, ModifiersState, NamedKey};

use crate::input::key_input::KeyInput;

pub fn translate_key(key: &Key, modifiers: ModifiersState) -> Option<KeyInput> {
    match key {
        Key::Named(NamedKey::ArrowLeft) if modifiers.alt_key() => Some(KeyInput::GoBack),
        Key::Named(NamedKey::ArrowRight) if modifiers.alt_key() => Some(KeyInput::GoForward),
        Key::Named(NamedKey::F5) => Some(KeyInput::Reload),
        Key::Named(NamedKey::F6) => Some(KeyInput::FocusAddress),
        Key::Character(text) if modifiers.control_key() && text.eq_ignore_ascii_case("r") => {
            Some(KeyInput::Reload)
        }
        Key::Character(text) if modifiers.control_key() && text.eq_ignore_ascii_case("l") => {
            Some(KeyInput::FocusAddress)
        }
        Key::Named(NamedKey::ArrowLeft) => Some(KeyInput::ArrowLeft),
        Key::Named(NamedKey::ArrowRight) => Some(KeyInput::ArrowRight),
        Key::Named(NamedKey::ArrowUp) => Some(KeyInput::ArrowUp),
        Key::Named(NamedKey::ArrowDown) => Some(KeyInput::ArrowDown),
        Key::Named(NamedKey::PageUp) => Some(KeyInput::PageUp),
        Key::Named(NamedKey::PageDown) => Some(KeyInput::PageDown),
        Key::Named(NamedKey::Home) => Some(KeyInput::Home),
        Key::Named(NamedKey::End) => Some(KeyInput::End),
        Key::Named(NamedKey::Space) => Some(KeyInput::Space),
        Key::Named(NamedKey::Enter) => Some(KeyInput::Enter),
        Key::Named(NamedKey::Backspace) => Some(KeyInput::Backspace),
        Key::Named(NamedKey::Delete) => Some(KeyInput::Delete),
        Key::Named(NamedKey::Escape) => Some(KeyInput::Escape),
        _ => None,
    }
}
