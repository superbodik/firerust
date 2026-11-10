use fr_bookmarks::model::bookmark_list::BookmarkList;
use fr_page::load::escape_html::escape_html;
use url::form_urlencoded::byte_serialize;

use crate::internal::page_style::PAGE_STYLE;

pub fn bookmarks_page(list: &BookmarkList) -> String {
    let mut html = format!(
        "<html><head><title>Bookmarks</title><style>{PAGE_STYLE}</style></head><body>\
         <h1>Bookmarks</h1>\
         <p><a href=\"about:import\">Import bookmarks</a></p>"
    );
    if list.is_empty() {
        html.push_str(
            "<p class=\"note\">No bookmarks yet. Press Ctrl+D on a page to add it, \
             or import them from another browser.</p>",
        );
    }
    for folder in list.folders() {
        let heading = if folder.is_empty() {
            "Unsorted"
        } else {
            folder.as_str()
        };
        html.push_str(&format!("<h2>{}</h2><ul>", escape_html(heading)));
        for item in list.items().iter().filter(|item| item.folder == folder) {
            let removal: String = byte_serialize(item.url.as_bytes()).collect();
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a> <a href=\"about:bookmarks?remove={}\">remove</a></li>",
                escape_html(&item.url),
                escape_html(&item.title),
                removal
            ));
        }
        html.push_str("</ul>");
    }
    html.push_str("</body></html>");
    html
}
