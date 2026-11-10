#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStripTarget {
    Tab(usize),
    Close(usize),
    New,
    Empty,
}
