use crate::tabs::window_control::WindowControl;
use crate::toolbar::toolbar_target::ToolbarTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChromeHover {
    #[default]
    Nothing,
    Tab(usize),
    TabClose(usize),
    NewTab,
    Control(WindowControl),
    Toolbar(ToolbarTarget),
}
