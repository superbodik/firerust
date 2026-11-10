#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKind {
    Text,
    Password,
    Checkbox,
    Radio,
    Button,
    Select,
    TextArea,
}
