use fr_page::load::escape_html::escape_html;

use crate::internal::group_html::group_html;
use crate::internal::page_html::page_html;
use crate::internal::row::Row;

pub fn history_page(visited: &[String], css: &str) -> String {
    let mut body = String::from(
        "<h1>History</h1>\
         <p class=\"lede\">Pages you opened in the tabs of this window, newest first.</p>",
    );
    if visited.is_empty() {
        body.push_str(
            "<div class=\"callout\"><p>Nothing yet.</p>\
             <p>Pages you open will be listed here until you close the browser.</p></div>",
        );
    } else {
        let rows: Vec<Row> = visited
            .iter()
            .map(|address| {
                let shown = escape_html(address);
                Row::text(&format!("<a class=\"grow\" href=\"{shown}\">{shown}</a>"))
            })
            .collect();
        body.push_str(&group_html(&rows));
    }
    page_html("History", css, &body)
}
