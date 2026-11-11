use url::Url;

use crate::load::escape_html::escape_html;
use crate::view::view_title::view_title;

pub fn image_view_html(url: &Url) -> String {
    let name = view_title(url);
    format!(
        "<html><head><title>{}</title></head>\
         <body style=\"margin: 0; background-color: #1f1d1b\">\
         <div style=\"padding: 24px; text-align: center\">\
         <img src=\"{}\" style=\"max-width: 100%\"></div></body></html>",
        escape_html(&name),
        escape_html(url.as_str())
    )
}
