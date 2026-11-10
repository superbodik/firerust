use fr_ui::address::address_field::AddressField;

pub struct ToolbarState<'a> {
    pub field: &'a AddressField,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}
