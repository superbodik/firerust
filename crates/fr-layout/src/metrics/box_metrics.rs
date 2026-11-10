use fr_css::value::edges::Edges;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxMetrics {
    pub margin: Edges<f32>,
    pub is_margin_left_auto: bool,
    pub is_margin_right_auto: bool,
    pub padding: Edges<f32>,
    pub border: Edges<f32>,
}

impl BoxMetrics {
    pub fn horizontal_extras(&self) -> f32 {
        self.padding.left + self.padding.right + self.border.left + self.border.right
    }

    pub fn vertical_extras(&self) -> f32 {
        self.padding.top + self.padding.bottom + self.border.top + self.border.bottom
    }
}
