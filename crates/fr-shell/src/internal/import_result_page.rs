use fr_page::load::escape_html::escape_html;

use crate::internal::page_html::page_html;

pub fn imported_page(found: usize, added: usize, css: &str) -> String {
    let already = found - added;
    page_html(
        "Import finished",
        css,
        &format!(
            "<h1>Import finished</h1>\
             <div class=\"card\"><p>Found {found} bookmarks, added {added} new ones \
             ({already} were already here).</p></div>\
             <p><a href=\"about:bookmarks\">Open bookmarks</a></p>"
        ),
    )
}

pub fn import_failed_page(reason: &str, css: &str) -> String {
    page_html(
        "Import failed",
        css,
        &format!(
            "<h1>Import failed</h1><div class=\"card\"><p>{}</p></div>\
             <p><a href=\"about:import\">Try again</a></p>",
            escape_html(reason)
        ),
    )
}
