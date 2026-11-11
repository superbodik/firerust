use fr_page::load::escape_html::escape_html;
use fr_passwords::model::credential::Credential;
use url::form_urlencoded::byte_serialize;

use crate::internal::row::Row;

const HIDDEN: &str = "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}";

fn encoded(text: &str) -> String {
    byte_serialize(text.as_bytes()).collect()
}

pub fn password_row(index: usize, login: &Credential, is_shown: bool) -> Row {
    let secret = if is_shown {
        escape_html(&login.password)
    } else {
        HIDDEN.to_owned()
    };
    let toggle = if is_shown {
        "<a href=\"about:passwords\">hide</a>".to_owned()
    } else {
        format!("<a href=\"about:passwords?show={index}\">show</a>")
    };
    Row::text(&format!(
        "<span class=\"grow\">{}<br><span class=\"note\">{} &middot; {}</span></span>\
         <span>{toggle} &nbsp; <a href=\"about:passwords?remove={}&amp;user={}\">remove</a></span>",
        escape_html(&login.origin),
        escape_html(&login.username),
        secret,
        encoded(&login.origin),
        encoded(&login.username)
    ))
}
