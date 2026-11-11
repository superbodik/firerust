#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CornerRadii {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl CornerRadii {
    pub const ZERO: Self = Self::uniform(0.0);

    pub const fn uniform(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    pub fn is_square(&self) -> bool {
        self.top_left <= 0.0
            && self.top_right <= 0.0
            && self.bottom_right <= 0.0
            && self.bottom_left <= 0.0
    }

    pub fn scaled(&self, factor: f32) -> Self {
        Self {
            top_left: self.top_left * factor,
            top_right: self.top_right * factor,
            bottom_right: self.bottom_right * factor,
            bottom_left: self.bottom_left * factor,
        }
    }

    pub fn fitted(&self, width: f32, height: f32) -> Self {
        let ratio = |room: f32, sum: f32| if sum > room { room / sum } else { 1.0 };
        let factor = ratio(width, self.top_left + self.top_right)
            .min(ratio(width, self.bottom_left + self.bottom_right))
            .min(ratio(height, self.top_left + self.bottom_left))
            .min(ratio(height, self.top_right + self.bottom_right))
            .max(0.0);
        self.scaled(factor)
    }

    pub fn inset(&self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        let shrink =
            |radius: f32, first: f32, second: f32| (radius - (first + second) / 2.0).max(0.0);
        Self {
            top_left: shrink(self.top_left, top, left),
            top_right: shrink(self.top_right, top, right),
            bottom_right: shrink(self.bottom_right, bottom, right),
            bottom_left: shrink(self.bottom_left, bottom, left),
        }
    }
}
