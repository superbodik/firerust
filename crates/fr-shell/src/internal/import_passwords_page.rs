use fr_page::load::escape_html::escape_html;

use crate::internal::page_html::page_html;

pub fn import_passwords_page(css: &str) -> String {
    page_html(
        "Import passwords",
        css,
        "<h1>Import passwords</h1>\
         <p><a href=\"about:passwords\">Back to passwords</a></p>\
         <div class=\"card\">\
         <p class=\"note\">In Chrome, Edge or Firefox open the password settings and export \
         your passwords to a CSV file, then type the full path of that file here. \
         The CSV file is plain text, so delete it as soon as the import is done.</p>\
         <form action=\"about:import-passwords\" method=\"get\">\
         <p><input type=\"text\" name=\"file\" size=\"60\"></p>\
         <button type=\"submit\">Import passwords</button></form></div>",
    )
}

pub fn passwords_imported_page(found: usize, added: usize, updated: usize, css: &str) -> String {
    page_html(
        "Import finished",
        css,
        &format!(
            "<h1>Import finished</h1>\
             <div class=\"card\"><p>Found {found} logins: {added} added, {updated} updated.</p></div>\
             <p class=\"note\">Now delete the CSV file: it holds your passwords as plain text.</p>\
             <p><a href=\"about:passwords\">Open passwords</a></p>"
        ),
    )
}

pub fn passwords_failed_page(reason: &str, css: &str) -> String {
    page_html(
        "Import failed",
        css,
        &format!(
            "<h1>Import failed</h1><div class=\"card\"><p>{}</p></div>\
             <p><a href=\"about:import-passwords\">Try again</a></p>",
            escape_html(reason)
        ),
    )
}
