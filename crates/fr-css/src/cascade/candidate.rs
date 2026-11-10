use crate::parse::declaration::Declaration;

pub struct Candidate<'a> {
    pub key: (bool, u8, u32, usize, usize),
    pub declaration: &'a Declaration,
}
