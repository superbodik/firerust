use crate::value::corner::Corner;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Corners<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl<T: Copy> Corners<T> {
    pub const fn uniform(value: T) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }

    pub fn get(&self, corner: Corner) -> T {
        match corner {
            Corner::TopLeft => self.top_left,
            Corner::TopRight => self.top_right,
            Corner::BottomRight => self.bottom_right,
            Corner::BottomLeft => self.bottom_left,
        }
    }

    pub fn set(&mut self, corner: Corner, value: T) {
        match corner {
            Corner::TopLeft => self.top_left = value,
            Corner::TopRight => self.top_right = value,
            Corner::BottomRight => self.bottom_right = value,
            Corner::BottomLeft => self.bottom_left = value,
        }
    }

    pub fn map<U>(self, convert: impl Fn(T) -> U) -> Corners<U> {
        Corners {
            top_left: convert(self.top_left),
            top_right: convert(self.top_right),
            bottom_right: convert(self.bottom_right),
            bottom_left: convert(self.bottom_left),
        }
    }
}
