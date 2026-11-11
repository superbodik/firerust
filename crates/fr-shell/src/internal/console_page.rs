use fr_page::load::escape_html::escape_html;

use crate::internal::group_html::group_html;
use crate::internal::page_html::page_html;
use crate::internal::row::Row;

fn line_row(line: &str) -> Row {
    let class = if line.starts_with("[error]") {
        "grow danger"
    } else {
        "grow"
    };
    Row::text(&format!(
        "<span class=\"{class}\">{}</span>",
        escape_html(line)
    ))
}

pub fn console_page(lines: &[String], css: &str) -> String {
    let mut body = String::from(
        "<h1>Console</h1>\
         <p class=\"lede\">What page scripts printed and which ones failed. \
         <a href=\"about:console?clear=1\">Clear the list</a></p>",
    );
    if lines.is_empty() {
        body.push_str(
            "<div class=\"callout\"><p>Nothing yet.</p>\
             <p>Messages from <b>console.log</b> and script errors show up here while you browse.</p></div>",
        );
    } else {
        let rows: Vec<Row> = lines.iter().rev().map(|line| line_row(line)).collect();
        body.push_str(&group_html(&rows));
    }
    page_html("Console", css, &body)
}
