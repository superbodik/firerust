use winit::keyboard::{Key, NamedKey};

use crate::input::key_input::KeyInput;

pub fn translate_key(key: &Key) -> Option<KeyInput> {
    match key {
        Key::Named(NamedKey::ArrowUp) => Some(KeyInput::ArrowUp),
        Key::Named(NamedKey::ArrowDown) => Some(KeyInput::ArrowDown),
        Key::Named(NamedKey::PageUp) => Some(KeyInput::PageUp),
        Key::Named(NamedKey::PageDown) => Some(KeyInput::PageDown),
        Key::Named(NamedKey::Home) => Some(KeyInput::Home),
        Key::Named(NamedKey::End) => Some(KeyInput::End),
        Key::Named(NamedKey::Space) => Some(KeyInput::Space),
        _ => None,
    }
}
