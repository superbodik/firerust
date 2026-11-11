pub struct Row {
    pub href: Option<String>,
    pub inner: String,
}

impl Row {
    pub fn text(inner: &str) -> Self {
        Self {
            href: None,
            inner: inner.to_owned(),
        }
    }

    pub fn link(href: &str, inner: &str) -> Self {
        Self {
            href: Some(href.to_owned()),
            inner: inner.to_owned(),
        }
    }
}
