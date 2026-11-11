use fr_bookmarks::model::bookmark_list::BookmarkList;
use fr_page::load::escape_html::escape_html;
use url::form_urlencoded::byte_serialize;

use crate::internal::group_html::group_html;
use crate::internal::page_html::page_html;
use crate::internal::row::Row;

pub fn bookmarks_page(list: &BookmarkList, css: &str) -> String {
    let mut body = String::from(
        "<h1>Bookmarks</h1>\
         <p class=\"lede\"><a href=\"about:import\">Import from another browser</a></p>",
    );
    if list.is_empty() {
        body.push_str(
            "<div class=\"callout\"><p>No bookmarks yet.</p>\
             <p>Press Ctrl+D on a page or click the star in the address bar. \
             You can also bring yours over from another browser.</p></div>",
        );
    }
    for folder in list.folders() {
        let heading = if folder.is_empty() {
            "Unsorted"
        } else {
            folder.as_str()
        };
        let rows: Vec<Row> = list
            .items()
            .iter()
            .filter(|item| item.folder == folder)
            .map(|item| {
                let removal: String = byte_serialize(item.url.as_bytes()).collect();
                Row::text(&format!(
                    "<a class=\"grow\" href=\"{}\">{}</a>\
                     <a class=\"hint\" href=\"about:bookmarks?remove={}\">remove</a>",
                    escape_html(&item.url),
                    escape_html(&item.title),
                    removal
                ))
            })
            .collect();
        body.push_str(&format!(
            "<h2>{}</h2>{}",
            escape_html(heading),
            group_html(&rows)
        ));
    }
    page_html("Bookmarks", css, &body)
}
