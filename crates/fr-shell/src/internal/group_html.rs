use crate::internal::row::Row;

pub fn group_html(rows: &[Row]) -> String {
    let mut html = String::from("<div class=\"group\">");
    for (index, row) in rows.iter().enumerate() {
        let class = if index == 0 { "row first" } else { "row" };
        match &row.href {
            Some(href) => html.push_str(&format!(
                "<a class=\"{class}\" href=\"{href}\">{}</a>",
                row.inner
            )),
            None => html.push_str(&format!("<div class=\"{class}\">{}</div>", row.inner)),
        }
    }
    html.push_str("</div>");
    html
}
