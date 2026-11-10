#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProcessMemory {
    pub resident_bytes: u64,
    pub virtual_bytes: u64,
}
