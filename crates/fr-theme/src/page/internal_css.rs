use fr_geom::color::rgba::Rgba;

use crate::colors::palette::Palette;

fn hex(color: Rgba) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
}

pub fn internal_css(palette: &Palette) -> String {
    format!(
        "body {{ background-color: {bg}; color: {text}; margin: 0; padding: 44px 28px 72px 28px; \
                accent-color: {accent} }} \
         .page {{ max-width: 620px; margin: 0 auto }} \
         .brand {{ font-size: 15px; font-weight: bold; margin: 0 0 44px 0; color: {text} }} \
         .brand span {{ color: {accent} }} \
         h1 {{ font-size: 28px; font-weight: bold; margin: 0 0 6px 0; color: {text} }} \
         h2 {{ font-size: 12px; font-weight: bold; margin: 34px 0 8px 2px; color: {muted} }} \
         p {{ margin: 8px 0; line-height: 1.55 }} \
         a {{ color: {link} }} \
         ul {{ margin: 0; padding: 0 }} \
         .lede {{ color: {muted}; margin: 0 0 26px 0 }} \
         .note {{ color: {muted} }} \
         .group {{ background-color: {card}; border: 1px solid {border}; border-radius: 10px; \
                   padding: 4px 0 }} \
         .row {{ display: flex; justify-content: space-between; align-items: center; \
                 padding: 11px 16px; border-top: 1px solid {border}; color: {text} }} \
         .row.first {{ border-top-width: 0 }} \
         .row .hint {{ color: {muted} }} \
         .row .grow {{ flex-grow: 1 }} \
         .row a {{ color: {link} }} \
         .keys {{ display: flex; gap: 4px }} \
         .key {{ display: block; padding: 1px 7px; background-color: {key_bg}; color: {text}; \
                 border: 1px solid {key_border}; border-bottom-width: 2px; border-radius: 5px; \
                 font-family: monospace; font-size: 12px }} \
         .callout {{ border-left: 3px solid {accent}; padding: 4px 0 4px 14px; margin: 22px 0; \
                     color: {muted} }} \
         .callout p {{ margin: 4px 0 }} \
         .title {{ font-size: 54px; font-weight: bold; color: {text}; margin: 76px 0 2px 0 }} \
         .title span {{ color: {accent} }} \
         .subtitle {{ color: {muted}; margin: 0 0 40px 0; font-size: 17px }} \
         .danger {{ color: {link} }} \
         input, select {{ background-color: {field}; color: {text}; border: 1px solid {border}; \
                          border-radius: 6px }} \
         input[type=\"radio\"] {{ border-radius: 50% }} \
         input[type=\"checkbox\"] {{ border-radius: 4px }} \
         button {{ background-color: {accent}; color: #ffffff; border: 1px solid {accent}; \
                   border-radius: 6px; font-weight: bold }}",
        bg = hex(palette.page_background),
        text = hex(palette.page_text),
        muted = hex(palette.page_muted),
        link = hex(palette.page_link),
        card = hex(palette.page_card),
        border = hex(palette.page_border),
        accent = hex(palette.accent),
        field = hex(palette.field_focused),
        key_bg = hex(palette.key_background),
        key_border = hex(palette.key_border),
    )
}
