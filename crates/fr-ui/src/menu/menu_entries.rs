use crate::menu::menu_entry::MenuEntry;
use crate::menu::menu_kind::MenuKind;
use crate::menu::menu_target::MenuTarget;

const MAIN: [MenuEntry; 15] = [
    MenuEntry::Item(MenuTarget::NewTab),
    MenuEntry::Item(MenuTarget::NewWindow),
    MenuEntry::Separator,
    MenuEntry::Item(MenuTarget::History),
    MenuEntry::Item(MenuTarget::Bookmarks),
    MenuEntry::Item(MenuTarget::Downloads),
    MenuEntry::Item(MenuTarget::Passwords),
    MenuEntry::Separator,
    MenuEntry::Zoom,
    MenuEntry::Separator,
    MenuEntry::Item(MenuTarget::Settings),
    MenuEntry::Item(MenuTarget::Console),
    MenuEntry::Item(MenuTarget::Pages),
    MenuEntry::Separator,
    MenuEntry::Item(MenuTarget::Exit),
];

const START: [MenuEntry; 3] = [
    MenuEntry::Item(MenuTarget::Bookmarks),
    MenuEntry::Item(MenuTarget::Passwords),
    MenuEntry::Item(MenuTarget::Pages),
];

pub fn menu_entries(kind: MenuKind) -> &'static [MenuEntry] {
    match kind {
        MenuKind::Main => &MAIN,
        MenuKind::Start => &START,
    }
}
