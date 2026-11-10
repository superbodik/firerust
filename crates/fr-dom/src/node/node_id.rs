use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(NonZeroU32);

impl NodeId {
    pub(crate) const ROOT: Self = Self(NonZeroU32::MIN);

    pub(crate) fn from_index(index: usize) -> Option<Self> {
        let number = u32::try_from(index).ok()?.checked_add(1)?;
        NonZeroU32::new(number).map(Self)
    }

    pub(crate) fn index(self) -> usize {
        self.0.get() as usize - 1
    }
}
