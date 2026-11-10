use fr_dom::tree::document::Document;
use fr_layout::control::control_kind::ControlKind;
use fr_layout::control::control_kind_of::control_kind_of;

use crate::login::login_fields::LoginFields;
use crate::state::focusable_controls::focusable_controls;
use crate::submit::form_of::form_of;

pub fn find_login_fields(document: &Document) -> Vec<LoginFields> {
    let controls = focusable_controls(document);
    let mut found = Vec::new();
    for (position, control) in controls.iter().enumerate() {
        if control_kind_of(document, *control) != Some(ControlKind::Password) {
            continue;
        }
        let form = form_of(document, *control);
        let username = controls[..position].iter().rev().copied().find(|earlier| {
            control_kind_of(document, *earlier) == Some(ControlKind::Text)
                && form_of(document, *earlier) == form
        });
        found.push(LoginFields {
            form,
            username,
            password: *control,
        });
    }
    found
}
