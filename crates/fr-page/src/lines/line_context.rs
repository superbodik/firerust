use crate::lines::backdrop::Backdrop;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LineContext {
    pub indent: f32,
    pub backdrop: Option<Backdrop>,
}
