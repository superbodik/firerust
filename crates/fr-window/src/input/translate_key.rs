use winit::keyboard::{Key, KeyCode, ModifiersState, NamedKey, PhysicalKey};

use crate::input::key_input::KeyInput;

pub fn translate_key(
    key: &Key,
    physical: PhysicalKey,
    modifiers: ModifiersState,
) -> Option<KeyInput> {
    if modifiers.control_key()
        && let Some(shortcut) = control_shortcut(physical)
    {
        return Some(shortcut);
    }
    match key {
        Key::Named(NamedKey::ArrowLeft) if modifiers.alt_key() => Some(KeyInput::GoBack),
        Key::Named(NamedKey::ArrowRight) if modifiers.alt_key() => Some(KeyInput::GoForward),
        Key::Named(NamedKey::F5) => Some(KeyInput::Reload),
        Key::Named(NamedKey::F6) => Some(KeyInput::FocusAddress),
        Key::Named(NamedKey::F9) => Some(KeyInput::ToggleReader),
        Key::Named(NamedKey::Tab) if modifiers.control_key() && modifiers.shift_key() => {
            Some(KeyInput::PreviousTab)
        }
        Key::Named(NamedKey::Tab) if modifiers.control_key() => Some(KeyInput::NextTab),
        Key::Named(NamedKey::Tab) if modifiers.shift_key() => Some(KeyInput::FocusPrevious),
        Key::Named(NamedKey::Tab) => Some(KeyInput::FocusNext),
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

fn control_shortcut(physical: PhysicalKey) -> Option<KeyInput> {
    let PhysicalKey::Code(code) = physical else {
        return None;
    };
    match code {
        KeyCode::KeyT => Some(KeyInput::NewTab),
        KeyCode::KeyW => Some(KeyInput::CloseTab),
        KeyCode::KeyL => Some(KeyInput::FocusAddress),
        KeyCode::KeyR => Some(KeyInput::Reload),
        KeyCode::KeyM => Some(KeyInput::ToggleMemoryPanel),
        KeyCode::KeyD => Some(KeyInput::ToggleBookmark),
        KeyCode::KeyB => Some(KeyInput::OpenBookmarks),
        KeyCode::KeyS => Some(KeyInput::SavePassword),
        KeyCode::KeyP => Some(KeyInput::OpenPasswords),
        KeyCode::KeyV => Some(KeyInput::Paste),
        KeyCode::KeyC => Some(KeyInput::Copy),
        KeyCode::KeyX => Some(KeyInput::Cut),
        KeyCode::KeyA => Some(KeyInput::SelectAll),
        KeyCode::KeyN => Some(KeyInput::NewWindow),
        KeyCode::KeyH => Some(KeyInput::OpenHistory),
        KeyCode::KeyJ => Some(KeyInput::OpenDownloads),
        KeyCode::Equal | KeyCode::NumpadAdd => Some(KeyInput::ZoomIn),
        KeyCode::Minus | KeyCode::NumpadSubtract => Some(KeyInput::ZoomOut),
        KeyCode::Digit0 | KeyCode::Numpad0 => Some(KeyInput::ZoomReset),
        _ => None,
    }
}
