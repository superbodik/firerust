#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadTarget {
    Initial,
    Navigate,
    Travel { scroll: f32 },
    Reload { scroll: f32 },
    Restore { scroll: f32 },
}
