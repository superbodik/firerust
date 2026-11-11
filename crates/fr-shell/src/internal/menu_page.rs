use crate::internal::group_html::group_html;
use crate::internal::keys_html::keys_html;
use crate::internal::page_html::page_html;
use crate::internal::row::Row;

fn shortcut(label: &str, keys: &str) -> Row {
    Row::text(&format!("<span>{label}</span>{}", keys_html(keys)))
}

fn destination(href: &str, label: &str, hint: &str) -> Row {
    Row::link(
        href,
        &format!("<span>{label}</span><span class=\"hint\">{hint}</span>"),
    )
}

pub fn menu_page(css: &str) -> String {
    let data = group_html(&[
        Row::link(
            "about:bookmarks",
            &format!("<span>Bookmarks</span>{}", keys_html("Ctrl+B")),
        ),
        Row::link(
            "about:passwords",
            &format!("<span>Passwords</span>{}", keys_html("Ctrl+P")),
        ),
    ]);
    let import = group_html(&[
        destination(
            "about:import",
            "Import bookmarks",
            "from a browser or a file",
        ),
        destination(
            "about:import-passwords",
            "Import passwords",
            "from a CSV export",
        ),
    ]);
    let keys = group_html(&[
        shortcut("New tab", "Ctrl+T"),
        shortcut("Close tab", "Ctrl+W"),
        shortcut("Next tab", "Ctrl+Tab"),
        shortcut("Address bar", "Ctrl+L"),
        shortcut("Bookmark this page", "Ctrl+D"),
        shortcut("Save the password you typed", "Ctrl+S"),
        shortcut("Zoom in", "Ctrl+Plus"),
        shortcut("Zoom out", "Ctrl+Minus"),
        shortcut("Reset zoom", "Ctrl+0"),
        shortcut("Memory panel", "Ctrl+M"),
        shortcut("Back", "Alt+Left"),
    ]);
    page_html(
        "Menu",
        css,
        &format!(
            "<h1>Menu</h1>\
             <p class=\"lede\">Everything lives behind these pages. The keys work anywhere.</p>\
             <h2>Your data</h2>{data}\
             <h2>Bring it over from another browser</h2>{import}\
             <h2>Keys</h2>{keys}"
        ),
    )
}
