use crate::lines::line::Line;
use crate::lines::line_context::LineContext;
use crate::lines::run::Run;
use crate::lines::run_style::RunStyle;

pub struct LineBuilder {
    lines: Vec<Line>,
    runs: Vec<Run>,
    line_context: LineContext,
    line_gap: f32,
    pending_gap: f32,
    has_pending_space: bool,
}

impl LineBuilder {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            runs: Vec::new(),
            line_context: LineContext::default(),
            line_gap: 0.0,
            pending_gap: 0.0,
            has_pending_space: false,
        }
    }

    pub fn push_text(&mut self, text: &str, style: RunStyle, context: LineContext) {
        let mut piece = String::new();
        for character in text.chars() {
            if character.is_whitespace() && character != '\u{a0}' {
                self.has_pending_space = true;
                continue;
            }
            if self.has_pending_space && (!self.runs.is_empty() || !piece.is_empty()) {
                piece.push(' ');
            }
            self.has_pending_space = false;
            piece.push(character);
        }
        if piece.is_empty() {
            return;
        }
        self.begin_line_if_empty(context);
        self.append(piece, style);
    }

    pub fn start_bullet(&mut self, style: RunStyle, context: LineContext) {
        self.break_line();
        self.begin_line_if_empty(context);
        self.append("\u{2022}".to_owned(), style);
        self.has_pending_space = true;
    }

    pub fn break_line(&mut self) {
        if !self.runs.is_empty() {
            self.lines.push(Line {
                runs: std::mem::take(&mut self.runs),
                indent: self.line_context.indent,
                gap_before: self.line_gap,
                background: self.line_context.backdrop,
            });
            self.line_gap = 0.0;
        }
        self.has_pending_space = false;
    }

    pub fn add_gap(&mut self, amount: f32) {
        self.pending_gap = self.pending_gap.max(amount);
    }

    pub fn finish(mut self) -> Vec<Line> {
        self.break_line();
        self.lines
    }

    fn begin_line_if_empty(&mut self, context: LineContext) {
        if self.runs.is_empty() {
            self.line_context = context;
            self.line_gap = std::mem::take(&mut self.pending_gap);
        }
    }

    fn append(&mut self, text: String, style: RunStyle) {
        match self.runs.last_mut() {
            Some(last) if last.style == style => last.text.push_str(&text),
            _ => self.runs.push(Run { text, style }),
        }
    }
}

impl Default for LineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
