pub fn page_html(title: &str, css: &str, body: &str) -> String {
    format!(
        "<html><head><title>{title}</title><style>{css}</style></head><body>\
         <div class=\"page\">{body}</div></body></html>"
    )
}
