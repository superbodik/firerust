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
         <p class=\"title\">firerust</p>\
         <p class=\"subtitle\">Type an address in the bar above and press Enter.</p>\
         <div class=\"links\">\
         <a href=\"about:bookmarks\">Bookmarks</a>\
         <a href=\"about:passwords\">Passwords</a>\
         <a href=\"about:menu\">Menu</a>\
         </div></div></body></html>"
    );
    Some(build_page(&html, url))
}
