use crate::value::side::Side;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Edges<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T: Copy> Edges<T> {
    pub const fn uniform(value: T) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn get(&self, side: Side) -> T {
        match side {
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
            Side::Left => self.left,
        }
    }

    pub fn set(&mut self, side: Side, value: T) {
        match side {
            Side::Top => self.top = value,
            Side::Right => self.right = value,
            Side::Bottom => self.bottom = value,
            Side::Left => self.left = value,
        }
    }

    pub fn map<U>(self, convert: impl Fn(T) -> U) -> Edges<U> {
        Edges {
            top: convert(self.top),
            right: convert(self.right),
            bottom: convert(self.bottom),
            left: convert(self.left),
        }
    }
}
