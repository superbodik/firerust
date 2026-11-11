pub fn keys_html(combination: &str) -> String {
    let mut html = String::from("<span class=\"keys\">");
    for part in combination.split('+') {
        html.push_str(&format!("<span class=\"key\">{part}</span>"));
    }
    html.push_str("</span>");
    html
}
