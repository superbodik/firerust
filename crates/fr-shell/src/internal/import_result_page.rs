use fr_page::load::escape_html::escape_html;

use crate::internal::page_style::PAGE_STYLE;

pub fn imported_page(found: usize, added: usize) -> String {
    let already = found - added;
    format!(
        "<html><head><title>Import finished</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Import finished</h1>\
         <p>Found {found} bookmarks, added {added} new ones ({already} were already here).</p>\
         <p><a href=\"about:bookmarks\">Open bookmarks</a></p></body></html>"
    )
}

pub fn import_failed_page(reason: &str) -> String {
    format!(
        "<html><head><title>Import failed</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Import failed</h1><p>{}</p>\
         <p><a href=\"about:import\">Try again</a></p></body></html>",
        escape_html(reason)
    )
}
