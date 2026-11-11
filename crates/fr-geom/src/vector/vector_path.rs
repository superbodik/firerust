use crate::vector::path_command::PathCommand;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VectorPath {
    pub commands: Vec<PathCommand>,
}

impl VectorPath {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, command: PathCommand) {
        self.commands.push(command);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}
