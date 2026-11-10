#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedImage {
    width: u32,
    height: u32,
    premultiplied_rgba: Vec<u8>,
}

impl DecodedImage {
    pub fn new(width: u32, height: u32, premultiplied_rgba: Vec<u8>) -> Self {
        Self {
            width,
            height,
            premultiplied_rgba,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn premultiplied_rgba(&self) -> &[u8] {
        &self.premultiplied_rgba
    }
}
