#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagRole {
    Keep,
    Unwrap,
    Drop,
}

const KEEP: [&str; 38] = [
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "li",
    "blockquote",
    "pre",
    "code",
    "em",
    "strong",
    "b",
    "i",
    "br",
    "hr",
    "table",
    "thead",
    "tbody",
    "tfoot",
    "tr",
    "td",
    "th",
    "caption",
    "figure",
    "figcaption",
    "dl",
    "dt",
    "dd",
    "sup",
    "sub",
    "a",
    "img",
    "q",
    "kbd",
];

const DROP: [&str; 24] = [
    "script", "style", "nav", "aside", "footer", "form", "button", "input", "select", "textarea",
    "iframe", "svg", "canvas", "noscript", "template", "menu", "dialog", "object", "embed",
    "video", "audio", "link", "meta", "head",
];

pub fn tag_role(name: &str) -> TagRole {
    if KEEP.contains(&name) {
        TagRole::Keep
    } else if DROP.contains(&name) {
        TagRole::Drop
    } else {
        TagRole::Unwrap
    }
}
