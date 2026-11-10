#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    None,
    #[default]
    Inline,
    Block,
    ListItem,
    Flex,
    Table,
    TableRowGroup,
    TableRow,
    TableCell,
}

impl Display {
    pub fn is_block_level(self) -> bool {
        !matches!(self, Self::None | Self::Inline)
    }
}
