#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneReaction {
    Idle,
    Redraw,
    DragWindow,
    Minimize,
    ToggleMaximize,
    CloseWindow,
}
