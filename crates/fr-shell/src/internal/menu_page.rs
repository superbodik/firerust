use crate::internal::page_html::page_html;

pub fn menu_page(css: &str) -> String {
    page_html(
        "Menu",
        css,
        "<h1>Menu</h1>\
         <p class=\"note\">Everything lives behind these pages. The shortcuts work anywhere.</p>\
         <h2>Your data</h2>\
         <div class=\"links\">\
         <a href=\"about:bookmarks\">Bookmarks (Ctrl+B)</a>\
         <a href=\"about:passwords\">Passwords (Ctrl+P)</a>\
         </div>\
         <h2>Bring it over from another browser</h2>\
         <div class=\"links\">\
         <a href=\"about:import\">Import bookmarks</a>\
         <a href=\"about:import-passwords\">Import passwords</a>\
         </div>\
         <h2>Shortcuts</h2>\
         <div class=\"card\">\
         <p>Ctrl+T new tab, Ctrl+W close tab, Ctrl+Tab next tab</p>\
         <p>Ctrl+D bookmark this page, Ctrl+S save the password you just typed</p>\
         <p>Ctrl+Plus, Ctrl+Minus, Ctrl+0 zoom, Ctrl+M memory panel</p>\
         </div>",
    )
}
