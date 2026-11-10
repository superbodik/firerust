#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    None,
    #[default]
    Inline,
    Block,
    ListItem,
}
