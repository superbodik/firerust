#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressOutcome {
    Edited,
    Submit,
    Cancel,
    Ignored,
}
