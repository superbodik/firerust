use fr_bookmarks::model::bookmark_list::BookmarkList;
use fr_page::load::escape_html::escape_html;
use url::form_urlencoded::byte_serialize;

use crate::internal::page_html::page_html;

pub fn bookmarks_page(list: &BookmarkList, css: &str) -> String {
    let mut body = String::from(
        "<h1>Bookmarks</h1>\
         <p><a href=\"about:import\">Import bookmarks</a> - <a href=\"about:menu\">Menu</a></p>",
    );
    if list.is_empty() {
        body.push_str(
            "<div class=\"card\"><p>No bookmarks yet.</p>\
             <p class=\"note\">Press Ctrl+D on a page or click the star in the address bar, \
             or import your bookmarks from another browser.</p></div>",
        );
    }
    for folder in list.folders() {
        let heading = if folder.is_empty() {
            "Unsorted"
        } else {
            folder.as_str()
        };
        body.push_str(&format!("<h2>{}</h2><ul>", escape_html(heading)));
        for item in list.items().iter().filter(|item| item.folder == folder) {
            let removal: String = byte_serialize(item.url.as_bytes()).collect();
            body.push_str(&format!(
                "<li><a href=\"{}\">{}</a> <span class=\"note\">- </span>\
                 <a href=\"about:bookmarks?remove={}\">remove</a></li>",
                escape_html(&item.url),
                escape_html(&item.title),
                removal
            ));
        }
        body.push_str("</ul>");
    }
    page_html("Bookmarks", css, &body)
}
