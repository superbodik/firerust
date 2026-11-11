use crate::menu::menu_target::MenuTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuHit {
    Outside,
    Inert,
    Target(MenuTarget),
}
