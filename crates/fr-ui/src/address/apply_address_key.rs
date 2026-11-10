use fr_window::input::key_input::KeyInput;

use crate::address::address_field::AddressField;
use crate::address::address_outcome::AddressOutcome;

pub fn apply_address_key(field: &mut AddressField, key: KeyInput) -> AddressOutcome {
    match key {
        KeyInput::Enter => AddressOutcome::Submit,
        KeyInput::Escape => AddressOutcome::Cancel,
        KeyInput::Backspace => edit(|| field.backspace()),
        KeyInput::Delete => edit(|| field.delete()),
        KeyInput::ArrowLeft => edit(|| field.move_left()),
        KeyInput::ArrowRight => edit(|| field.move_right()),
        KeyInput::Home => edit(|| field.move_home()),
        KeyInput::End => edit(|| field.move_end()),
        KeyInput::Space => edit(|| field.insert(' ')),
        _ => AddressOutcome::Ignored,
    }
}

fn edit(action: impl FnOnce()) -> AddressOutcome {
    action();
    AddressOutcome::Edited
}
