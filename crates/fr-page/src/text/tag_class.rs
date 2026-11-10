const HIDDEN: &[&str] = &["head", "script", "style", "template", "noscript", "title"];

const BLOCK: &[&str] = &[
    "html",
    "body",
    "div",
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
    "table",
    "thead",
    "tbody",
    "tfoot",
    "tr",
    "section",
    "article",
    "header",
    "footer",
    "nav",
    "main",
    "aside",
    "blockquote",
    "pre",
    "form",
    "fieldset",
    "hr",
    "dl",
    "dt",
    "dd",
    "address",
    "figure",
    "figcaption",
    "details",
    "summary",
    "br",
];

const GAP: &[&str] = &[
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "blockquote",
    "pre",
    "table",
];

pub fn is_hidden(name: &str) -> bool {
    HIDDEN.contains(&name)
}

pub fn is_block(name: &str) -> bool {
    BLOCK.contains(&name)
}

pub fn has_gap(name: &str) -> bool {
    GAP.contains(&name)
}
