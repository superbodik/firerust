use fr_bookmarks::import::browser_source::BrowserSource;
use fr_page::load::escape_html::escape_html;

use crate::internal::page_style::PAGE_STYLE;

pub fn import_page() -> String {
    let mut html = format!(
        "<html><head><title>Import bookmarks</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Import bookmarks</h1>\
         <p><a href=\"about:bookmarks\">Back to bookmarks</a></p>\
         <h2>From a browser on this computer</h2>\
         <form action=\"about:import\" method=\"get\">"
    );
    for (index, source) in BrowserSource::ALL.iter().enumerate() {
        let checked = if index == 0 { " checked" } else { "" };
        html.push_str(&format!(
            "<p><input type=\"radio\" name=\"source\" value=\"{}\"{}> {}</p>",
            source.key(),
            checked,
            escape_html(source.label())
        ));
    }
    html.push_str(
        "<button type=\"submit\">Import from this browser</button></form>\
         <h2>From a file</h2>\
         <p class=\"note\">An HTML export (Firefox, Chrome, Edge and Safari can all make one) \
         or a Chromium Bookmarks file. Type the full path:</p>\
         <form action=\"about:import\" method=\"get\">\
         <p><input type=\"text\" name=\"file\" size=\"60\"></p>\
         <button type=\"submit\">Import from the file</button></form>\
         </body></html>",
    );
    html
}
