use fr_page::load::escape_html::escape_html;

use crate::internal::page_style::PAGE_STYLE;

pub fn import_passwords_page() -> String {
    format!(
        "<html><head><title>Import passwords</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Import passwords</h1>\
         <p><a href=\"about:passwords\">Back to passwords</a></p>\
         <p class=\"note\">In Chrome, Edge or Firefox open the password settings and export \
         your passwords to a CSV file, then type the full path of that file here. \
         The CSV file is plain text, so delete it as soon as the import is done.</p>\
         <form action=\"about:import-passwords\" method=\"get\">\
         <p><input type=\"text\" name=\"file\" size=\"60\"></p>\
         <button type=\"submit\">Import passwords</button></form>\
         </body></html>"
    )
}

pub fn passwords_imported_page(found: usize, added: usize, updated: usize) -> String {
    format!(
        "<html><head><title>Import finished</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Import finished</h1>\
         <p>Found {found} logins: {added} added, {updated} updated.</p>\
         <p class=\"note\">Now delete the CSV file: it holds your passwords as plain text.</p>\
         <p><a href=\"about:passwords\">Open passwords</a></p></body></html>"
    )
}

pub fn passwords_failed_page(reason: &str) -> String {
    format!(
        "<html><head><title>Import failed</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Import failed</h1><p>{}</p>\
         <p><a href=\"about:import-passwords\">Try again</a></p></body></html>",
        escape_html(reason)
    )
}
