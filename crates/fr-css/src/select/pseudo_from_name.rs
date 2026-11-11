use crate::select::pseudo_class::PseudoClass;

pub fn pseudo_from_name(name: &str) -> Option<PseudoClass> {
    match name.to_ascii_lowercase().as_str() {
        "link" | "any-link" => Some(PseudoClass::Link),
        "root" => Some(PseudoClass::Root),
        "first-child" => Some(PseudoClass::FirstChild),
        "last-child" => Some(PseudoClass::LastChild),
        "only-child" => Some(PseudoClass::OnlyChild),
        "empty" => Some(PseudoClass::Empty),
        "checked" => Some(PseudoClass::Checked),
        _ => None,
    }
}
