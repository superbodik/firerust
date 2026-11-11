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
         <p class=\"subtitle\">A small browser. Search or type an address.</p>\
         <form action=\"about:search\" method=\"get\">\
         <input type=\"text\" name=\"q\" class=\"search\" \
         placeholder=\"Search the web or type an address\">\
         </form>\
         <p class=\"note\">Press Enter to go. Your shortcuts are in the corner.</p>\
         </div></body></html>"
    );
    Some(build_page(&html, url))
}
