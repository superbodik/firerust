use crate::lines::run_style::RunStyle;

#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub text: String,
    pub style: RunStyle,
}
