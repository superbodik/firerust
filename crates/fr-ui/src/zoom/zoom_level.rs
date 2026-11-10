const STEPS: [u16; 17] = [
    25, 33, 50, 67, 75, 80, 90, 100, 110, 125, 150, 175, 200, 250, 300, 400, 500,
];
const DEFAULT_PERCENT: u16 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoomLevel {
    percent: u16,
}

impl ZoomLevel {
    pub const fn new() -> Self {
        Self {
            percent: DEFAULT_PERCENT,
        }
    }

    pub const fn percent(self) -> u16 {
        self.percent
    }

    pub fn factor(self) -> f32 {
        f32::from(self.percent) / 100.0
    }

    pub const fn is_default(self) -> bool {
        self.percent == DEFAULT_PERCENT
    }

    pub fn zoom_in(self) -> Self {
        let next = STEPS
            .iter()
            .copied()
            .find(|step| *step > self.percent)
            .unwrap_or(self.percent);
        Self { percent: next }
    }

    pub fn zoom_out(self) -> Self {
        let next = STEPS
            .iter()
            .rev()
            .copied()
            .find(|step| *step < self.percent)
            .unwrap_or(self.percent);
        Self { percent: next }
    }
}

impl Default for ZoomLevel {
    fn default() -> Self {
        Self::new()
    }
}
