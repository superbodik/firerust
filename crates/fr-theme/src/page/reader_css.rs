use fr_geom::color::rgba::Rgba;

use crate::colors::palette::Palette;

fn hex(color: Rgba) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
}

pub fn reader_css(palette: &Palette) -> String {
    format!(
        "body {{ background-color: {bg}; color: {text}; margin: 0; padding: 36px 24px 90px 24px }} \
         .reader {{ max-width: 680px; margin: 0 auto; font-size: 18px; line-height: 1.7 }} \
         .origin {{ color: {muted}; font-size: 13px; margin: 0 0 16px 0 }} \
         h1 {{ font-size: 32px; line-height: 1.25; margin: 0 0 26px 0; color: {text} }} \
         h2 {{ font-size: 25px; line-height: 1.3; margin: 36px 0 10px 0 }} \
         h3 {{ font-size: 21px; margin: 28px 0 8px 0 }} \
         h4, h5, h6 {{ font-size: 18px; margin: 22px 0 6px 0 }} \
         p {{ margin: 0 0 18px 0 }} \
         a {{ color: {link} }} \
         img {{ max-width: 100% }} \
         blockquote {{ border-left: 3px solid {accent}; margin: 20px 0; padding: 2px 0 2px 18px; \
                       color: {muted} }} \
         pre {{ background-color: {card}; border: 1px solid {border}; border-radius: 8px; \
                padding: 14px; font-size: 14px; line-height: 1.5; margin: 0 0 20px 0 }} \
         code {{ font-family: monospace; font-size: 16px }} \
         ul, ol {{ margin: 0 0 18px 0; padding-left: 28px }} \
         li {{ margin: 0 0 6px 0 }} \
         table {{ margin: 0 0 20px 0 }} \
         td, th {{ padding: 6px 12px 6px 0; border-bottom: 1px solid {border} }} \
         figure {{ margin: 24px 0 }} \
         figcaption {{ color: {muted}; font-size: 14px; margin: 6px 0 0 0 }} \
         hr {{ margin: 28px 0 }}",
        bg = hex(palette.page_background),
        text = hex(palette.page_text),
        muted = hex(palette.page_muted),
        link = hex(palette.page_link),
        card = hex(palette.page_card),
        border = hex(palette.page_border),
        accent = hex(palette.accent),
    )
}
