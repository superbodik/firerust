use crate::menu::menu_target::MenuTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEntry {
    Item(MenuTarget),
    Separator,
    Zoom,
}
