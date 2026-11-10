use crate::lines::backdrop::Backdrop;
use crate::lines::run::Run;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub runs: Vec<Run>,
    pub indent: f32,
    pub gap_before: f32,
    pub background: Option<Backdrop>,
}

impl Line {
    pub fn plain_text(&self) -> String {
        self.runs.iter().map(|run| run.text.as_str()).collect()
    }
}
