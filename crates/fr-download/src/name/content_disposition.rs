#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContentDisposition {
    pub is_attachment: bool,
    pub file_name: Option<String>,
}
