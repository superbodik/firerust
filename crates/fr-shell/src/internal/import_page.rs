use fr_bookmarks::import::browser_source::BrowserSource;
use fr_page::load::escape_html::escape_html;

use crate::internal::page_html::page_html;

pub fn import_page(css: &str) -> String {
    let mut body = String::from(
        "<h1>Import bookmarks</h1>\
         <p class=\"lede\"><a href=\"about:bookmarks\">Back to bookmarks</a></p>\
         <h2>From a browser on this computer</h2>\
         <form action=\"about:import\" method=\"get\"><div class=\"group\">",
    );
    for (index, source) in BrowserSource::ALL.iter().enumerate() {
        let class = if index == 0 { "row first" } else { "row" };
        let checked = if index == 0 { " checked" } else { "" };
        body.push_str(&format!(
            "<div class=\"{class}\"><span class=\"grow\">{}</span>\
             <input type=\"radio\" name=\"source\" value=\"{}\"{checked}></div>",
            escape_html(source.label()),
            source.key()
        ));
    }
    body.push_str(
        "</div><p><button type=\"submit\">Import from this browser</button></p></form>\
         <h2>From a file</h2>\
         <p class=\"note\">An HTML export (Firefox, Chrome, Edge and Safari can all make one) \
         or a Chromium Bookmarks file. Type the full path.</p>\
         <form action=\"about:import\" method=\"get\">\
         <p><input type=\"text\" name=\"file\" size=\"58\"></p>\
         <p><button type=\"submit\">Import from the file</button></p></form>",
    );
    page_html("Import bookmarks", css, &body)
}
