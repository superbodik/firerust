use fr_theme::colors::palette_for::palette_for;
use fr_theme::kind::theme_kind::ThemeKind;
use fr_theme::page::internal_css::internal_css;
use url::Url;

use crate::load::build_page::build_page;
use crate::load::page::Page;

pub const BLANK_ADDRESS: &str = "about:blank";

pub fn blank_page() -> Option<Page> {
    blank_page_for(ThemeKind::Light)
}

pub fn blank_page_for(theme: ThemeKind) -> Option<Page> {
    let url = Url::parse(BLANK_ADDRESS).ok()?;
    let css = internal_css(palette_for(theme));
    let html = format!(
        "<html><head><title>New tab</title><style>{css}</style></head><body>\
         <div class=\"page\">\
         <p class=\"title\">fire<span>rust</span></p>\
         <p class=\"subtitle\">A small browser. Type an address above and press Enter.</p>\
         <div class=\"group\">\
         <a class=\"row first\" href=\"about:bookmarks\"><span>Bookmarks</span>\
         <span class=\"keys\"><span class=\"key\">Ctrl</span><span class=\"key\">B</span></span></a>\
         <a class=\"row\" href=\"about:passwords\"><span>Passwords</span>\
         <span class=\"keys\"><span class=\"key\">Ctrl</span><span class=\"key\">P</span></span></a>\
         <a class=\"row\" href=\"about:menu\"><span>Menu</span>\
         <span class=\"hint\">all pages and keys</span></a>\
         </div></div></body></html>"
    );
    Some(build_page(&html, url))
}
