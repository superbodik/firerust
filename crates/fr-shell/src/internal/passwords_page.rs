use fr_page::load::escape_html::escape_html;
use fr_passwords::model::password_list::PasswordList;
use url::form_urlencoded::byte_serialize;

use crate::internal::page_style::PAGE_STYLE;

const HIDDEN: &str = "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}";

fn encoded(text: &str) -> String {
    byte_serialize(text.as_bytes()).collect()
}

pub fn passwords_page(list: &PasswordList, shown: Option<usize>) -> String {
    let mut html = format!(
        "<html><head><title>Passwords</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Passwords</h1>\
         <p><a href=\"about:import-passwords\">Import passwords</a></p>\
         <p class=\"note\">Passwords are kept in an encrypted file that only your Windows account can open.</p>"
    );
    if list.is_empty() {
        html.push_str(
            "<p class=\"note\">No saved passwords yet. After you sign in on a site, press Ctrl+S \
             to save the login, or import a CSV export from another browser.</p>",
        );
    }
    html.push_str("<ul>");
    for (index, login) in list.items().iter().enumerate() {
        let secret = if shown == Some(index) {
            escape_html(&login.password)
        } else {
            HIDDEN.to_owned()
        };
        let toggle = if shown == Some(index) {
            "<a href=\"about:passwords\">hide</a>".to_owned()
        } else {
            format!("<a href=\"about:passwords?show={index}\">show</a>")
        };
        html.push_str(&format!(
            "<li>{} - {} - {} {} <a href=\"about:passwords?remove={}&amp;user={}\">remove</a></li>",
            escape_html(&login.origin),
            escape_html(&login.username),
            secret,
            toggle,
            encoded(&login.origin),
            encoded(&login.username)
        ));
    }
    html.push_str("</ul></body></html>");
    html
}
