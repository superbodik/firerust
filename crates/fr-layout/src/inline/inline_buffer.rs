use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::white_space::WhiteSpace;

use crate::boxes::layout_run::LayoutRun;

#[derive(Debug, Default)]
pub struct InlineBuffer {
    runs: Vec<LayoutRun>,
    has_pending_space: bool,
    is_after_break: bool,
}

impl InlineBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_text(&mut self, text: &str, style: &ComputedStyle) {
        if style.white_space == WhiteSpace::Pre {
            self.push_preformatted(text, style);
            return;
        }
        let mut piece = String::new();
        for character in text.chars() {
            if character.is_whitespace() && character != '\u{a0}' {
                self.has_pending_space = true;
                continue;
            }
            if self.has_pending_space && self.has_text_on_line(&piece) {
                piece.push(' ');
            }
            self.has_pending_space = false;
            piece.push(character);
        }
        self.append(piece, style);
    }

    pub fn push_break(&mut self, style: &ComputedStyle) {
        self.append("\n".to_owned(), style);
        self.has_pending_space = false;
        self.is_after_break = true;
    }

    pub fn into_runs(mut self) -> Vec<LayoutRun> {
        if let Some(last) = self.runs.last_mut() {
            let trimmed = last.text.trim_end_matches(' ').len();
            last.text.truncate(trimmed);
            if last.text.is_empty() {
                self.runs.pop();
            }
        }
        let has_content = self.runs.iter().any(|run| !run.text.trim().is_empty());
        if has_content { self.runs } else { Vec::new() }
    }

    fn push_preformatted(&mut self, text: &str, style: &ComputedStyle) {
        self.has_pending_space = false;
        self.append(text.to_owned(), style);
        self.is_after_break = text.ends_with('\n');
    }

    fn has_text_on_line(&self, piece: &str) -> bool {
        !piece.is_empty() || (!self.runs.is_empty() && !self.is_after_break)
    }

    fn append(&mut self, text: String, style: &ComputedStyle) {
        if text.is_empty() {
            return;
        }
        if text != "\n" {
            self.is_after_break = false;
        }
        let run = LayoutRun::from_style(text, style);
        match self.runs.last_mut() {
            Some(last) if last.has_same_style(&run) => last.text.push_str(&run.text),
            _ => self.runs.push(run),
        }
    }
}
