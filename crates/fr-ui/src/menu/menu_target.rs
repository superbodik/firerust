#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuTarget {
    NewTab,
    NewWindow,
    History,
    Bookmarks,
    Downloads,
    Passwords,
    ZoomOut,
    ZoomIn,
    ZoomReset,
    Settings,
    Console,
    Pages,
    Reader,
    PlainStyles,
    Exit,
}
