use fr_dom::node::node_id::NodeId;
use fr_layout::control::control_focus::ControlFocus;
use fr_layout::control::control_kind::ControlKind;
use fr_tab::set::tab_set::TabSet;
use fr_ui::address::address_field::AddressField;
use fr_ui::address::address_outcome::AddressOutcome;
use fr_ui::address::apply_address_key::apply_address_key;
use fr_window::input::key_input::KeyInput;

use crate::scenes::control_kind_in::control_kind_in;
use crate::scenes::dispatch_event_to::dispatch_event_to;
use crate::scenes::focus_key::FocusKey;
use crate::scenes::invalidate_layout::invalidate_layout;
use crate::scenes::read_control_text::read_control_text;
use crate::scenes::write_control_text::write_control_text;

pub struct FormFocus {
    control: Option<NodeId>,
    field: AddressField,
}

impl FormFocus {
    pub fn new() -> Self {
        Self {
            control: None,
            field: AddressField::new(),
        }
    }

    pub fn control(&self) -> Option<NodeId> {
        self.control
    }

    pub fn layout_focus(&self) -> Option<ControlFocus> {
        self.control.map(|node| ControlFocus {
            node,
            caret: self.field.caret(),
        })
    }

    pub fn forget(&mut self) {
        self.control = None;
    }

    pub fn focus(&mut self, tabs: &mut TabSet, control: NodeId) {
        let text = read_control_text(tabs, control);
        self.field.set_text(&text);
        self.control = Some(control);
        invalidate_layout(tabs);
    }

    pub fn blur(&mut self, tabs: &mut TabSet) -> bool {
        if self.control.take().is_none() {
            return false;
        }
        invalidate_layout(tabs);
        true
    }

    pub fn type_char(&mut self, tabs: &mut TabSet, character: char) -> bool {
        let Some(control) = self.control else {
            return false;
        };
        if !is_text_kind(control_kind_in(tabs, control)) {
            return false;
        }
        self.field.insert(character);
        self.commit(tabs, control);
        true
    }

    pub fn paste(&mut self, tabs: &mut TabSet, text: &str) -> bool {
        let mut inserted = false;
        for character in text.chars() {
            inserted |= self.type_char(tabs, character);
        }
        inserted
    }

    pub fn current_text(&self) -> Option<String> {
        self.control?;
        Some(self.field.text().to_owned())
    }

    pub fn press(&mut self, tabs: &mut TabSet, key: KeyInput) -> FocusKey {
        let Some(control) = self.control else {
            return FocusKey::Ignored;
        };
        let kind = control_kind_in(tabs, control);
        match kind {
            Some(ControlKind::Text | ControlKind::Password) => {
                self.press_in_text(tabs, control, key, false)
            }
            Some(ControlKind::TextArea) => self.press_in_text(tabs, control, key, true),
            Some(_) => match key {
                KeyInput::Space | KeyInput::Enter => FocusKey::Activate(control),
                KeyInput::Escape => {
                    self.blur(tabs);
                    FocusKey::Blurred
                }
                _ => FocusKey::Ignored,
            },
            None => FocusKey::Ignored,
        }
    }

    fn press_in_text(
        &mut self,
        tabs: &mut TabSet,
        control: NodeId,
        key: KeyInput,
        is_multiline: bool,
    ) -> FocusKey {
        match key {
            KeyInput::Enter if is_multiline => {
                self.field.insert('\n');
                self.commit(tabs, control);
                FocusKey::Edited
            }
            KeyInput::Enter => FocusKey::SubmitFrom(control),
            KeyInput::Escape => {
                self.blur(tabs);
                FocusKey::Blurred
            }
            other => match apply_address_key(&mut self.field, other) {
                AddressOutcome::Edited => {
                    self.commit(tabs, control);
                    FocusKey::Edited
                }
                _ => FocusKey::Ignored,
            },
        }
    }

    fn commit(&mut self, tabs: &mut TabSet, control: NodeId) {
        let text = self.field.text().to_owned();
        let previous = read_control_text(tabs, control);
        if previous != text {
            write_control_text(tabs, control, &text);
            dispatch_event_to(tabs, control, "input");
        }
        invalidate_layout(tabs);
    }
}

impl Default for FormFocus {
    fn default() -> Self {
        Self::new()
    }
}

fn is_text_kind(kind: Option<ControlKind>) -> bool {
    matches!(
        kind,
        Some(ControlKind::Text | ControlKind::Password | ControlKind::TextArea)
    )
}
