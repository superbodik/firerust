use fr_dom::node::node_id::NodeId;
use fr_form::state::attribute_of::attribute_of;
use fr_form::state::choose_radio::choose_radio;
use fr_form::state::cycle_select::cycle_select;
use fr_form::state::toggle_checkbox::toggle_checkbox;
use fr_form::submit::form_of::form_of;
use fr_layout::control::control_kind::ControlKind;
use fr_layout::control::control_kind_of::control_kind_of;
use fr_tab::set::tab_set::TabSet;

use crate::scenes::control_action::ControlAction;

pub fn click_control(tabs: &mut TabSet, control: NodeId) -> ControlAction {
    let Some(content) = tabs.active_mut().content_mut() else {
        return ControlAction::Nothing;
    };
    let shared = content.document.clone();
    let mut document = shared.borrow_mut();
    let Some(kind) = control_kind_of(&document, control) else {
        return ControlAction::Nothing;
    };
    match kind {
        ControlKind::Text | ControlKind::Password | ControlKind::TextArea => {
            ControlAction::Focus(control)
        }
        ControlKind::Checkbox => changed(toggle_checkbox(&mut document, control).is_ok(), control),
        ControlKind::Radio => changed(choose_radio(&mut document, control).is_ok(), control),
        ControlKind::Select => changed(cycle_select(&mut document, control).is_ok(), control),
        ControlKind::Button => {
            let declared = attribute_of(&document, control, "type")
                .unwrap_or_else(|| "submit".to_owned())
                .to_ascii_lowercase();
            if declared != "submit" && declared != "image" {
                return ControlAction::Nothing;
            }
            match form_of(&document, control) {
                Some(form) => ControlAction::Submit {
                    form,
                    submitter: control,
                },
                None => ControlAction::Nothing,
            }
        }
    }
}

fn changed(succeeded: bool, control: NodeId) -> ControlAction {
    if succeeded {
        ControlAction::Changed(control)
    } else {
        ControlAction::Nothing
    }
}
