use fr_text::render::text_alignment::TextAlignment;
use fr_text::render::text_run::TextRun;

use crate::boxes::layout_run::LayoutRun;

#[derive(Debug, Clone, PartialEq)]
pub struct TextBox {
    pub runs: Vec<LayoutRun>,
    pub alignment: TextAlignment,
}

impl TextBox {
    pub fn text_runs(&self) -> Vec<TextRun<'_>> {
        self.runs.iter().map(LayoutRun::as_text_run).collect()
    }

    pub fn plain_text(&self) -> String {
        self.runs.iter().map(|run| run.text.as_str()).collect()
    }
}
