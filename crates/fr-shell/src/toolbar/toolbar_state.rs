use fr_ui::address::address_field::AddressField;
use fr_ui::hover::chrome_hover::ChromeHover;

use crate::toolbar::security_badge::SecurityBadge;

pub struct ToolbarState<'a> {
    pub field: &'a AddressField,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub is_loading: bool,
    pub is_bookmarked: bool,
    pub badge: SecurityBadge,
    pub hover: ChromeHover,
}
