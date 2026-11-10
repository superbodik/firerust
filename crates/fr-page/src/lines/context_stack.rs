use fr_geom::color::rgba::Rgba;

use crate::lines::backdrop::Backdrop;
use crate::lines::line_context::LineContext;

pub struct ContextStack {
    stack: Vec<LineContext>,
}

impl ContextStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn current(&self) -> LineContext {
        self.stack.last().copied().unwrap_or_default()
    }

    pub fn push(&mut self, margin_left: f32, inset_left: f32, background: Option<Rgba>) {
        let base = self.current();
        let border_left = base.indent + margin_left;
        let backdrop = background
            .map(|color| Backdrop {
                color,
                left: border_left,
            })
            .or(base.backdrop);
        self.stack.push(LineContext {
            indent: border_left + inset_left,
            backdrop,
        });
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}

impl Default for ContextStack {
    fn default() -> Self {
        Self::new()
    }
}
