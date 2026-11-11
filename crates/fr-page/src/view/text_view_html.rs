use crate::load::escape_html::escape_html;

const MAX_CHARACTERS: usize = 2_000_000;

pub fn text_view_html(title: &str, text: &str) -> String {
    let shown: String = text.chars().take(MAX_CHARACTERS).collect();
    let cut = if shown.len() < text.len() {
        "\n\n[The rest of this file is not shown]"
    } else {
        ""
    };
    format!(
        "<html><head><title>{}</title></head><body style=\"margin: 0\">\
         <pre style=\"margin: 14px 16px; font-size: 14px\">{}{}</pre></body></html>",
        escape_html(title),
        escape_html(&shown),
        cut
    )
}
