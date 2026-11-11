use fr_passwords::model::password_list::PasswordList;

use crate::internal::group_html::group_html;
use crate::internal::page_html::page_html;
use crate::internal::password_row::password_row;
use crate::internal::row::Row;

pub fn passwords_page(list: &PasswordList, shown: Option<usize>, css: &str) -> String {
    let mut body = String::from(
        "<h1>Passwords</h1>\
         <p class=\"lede\"><a href=\"about:import-passwords\">Import from a CSV file</a></p>",
    );
    if list.is_empty() {
        body.push_str(
            "<div class=\"callout\"><p>No saved passwords yet.</p>\
             <p>After you sign in on a site press Ctrl+S to keep the login, \
             or import a CSV export from another browser.</p></div>",
        );
    } else {
        let rows: Vec<Row> = list
            .items()
            .iter()
            .enumerate()
            .map(|(index, login)| password_row(index, login, shown == Some(index)))
            .collect();
        body.push_str(&group_html(&rows));
    }
    body.push_str(
        "<div class=\"callout\"><p>The list is kept in a file encrypted for your Windows \
         account. Nobody else can open it.</p></div>",
    );
    page_html("Passwords", css, &body)
}
