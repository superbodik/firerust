use fr_icons::catalog::icon::Icon;
use fr_ui::menu::menu_target::MenuTarget;

pub struct MenuLabel {
    pub icon: Icon,
    pub text: &'static str,
    pub shortcut: &'static str,
}

pub fn menu_label(target: MenuTarget) -> MenuLabel {
    let (icon, text, shortcut) = match target {
        MenuTarget::NewTab => (Icon::Plus, "New tab", "Ctrl+T"),
        MenuTarget::NewWindow => (Icon::Restore, "New window", "Ctrl+N"),
        MenuTarget::History => (Icon::ClockHistory, "History", "Ctrl+H"),
        MenuTarget::Bookmarks => (Icon::Bookmarks, "Bookmarks", "Ctrl+B"),
        MenuTarget::Downloads => (Icon::Download, "Downloads", "Ctrl+J"),
        MenuTarget::Passwords => (Icon::Key, "Passwords", "Ctrl+P"),
        MenuTarget::Reader => (Icon::Book, "Reader view", "F9"),
        MenuTarget::PlainStyles => (Icon::Brush, "Page without styles", ""),
        MenuTarget::Settings => (Icon::Gear, "Settings", ""),
        MenuTarget::Console => (Icon::Terminal, "Console", ""),
        MenuTarget::Pages => (Icon::House, "All pages and keys", ""),
        MenuTarget::Exit => (Icon::Power, "Exit", ""),
        MenuTarget::ZoomOut => (Icon::Minimize, "Zoom out", ""),
        MenuTarget::ZoomIn => (Icon::Plus, "Zoom in", ""),
        MenuTarget::ZoomReset => (Icon::Search, "Reset zoom", ""),
    };
    MenuLabel {
        icon,
        text,
        shortcut,
    }
}
