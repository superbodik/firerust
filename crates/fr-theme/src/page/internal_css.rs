use fr_geom::color::rgba::Rgba;

use crate::colors::palette::Palette;

fn hex(color: Rgba) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
}

pub fn internal_css(palette: &Palette) -> String {
    format!(
        "body {{ background-color: {bg}; color: {text}; margin: 0; padding: 40px 24px }} \
         .page {{ max-width: 720px; margin: 0 auto }} \
         h1 {{ font-size: 30px; margin: 0 0 6px 0; color: {text} }} \
         h2 {{ font-size: 15px; margin: 28px 0 8px 0; color: {muted} }} \
         p {{ margin: 8px 0; line-height: 1.5 }} \
         a {{ color: {link} }} \
         ul {{ margin: 0; padding: 0 }} \
         li {{ margin: 0 0 6px 0; padding: 10px 14px; background-color: {card}; \
               border: 1px solid {border}; border-radius: 10px }} \
         .note {{ color: {muted} }} \
         .card {{ background-color: {card}; border: 1px solid {border}; border-radius: 12px; \
                  padding: 14px 18px; margin: 10px 0 }} \
         .links a {{ display: block; padding: 10px 14px; margin: 0 0 6px 0; \
                     background-color: {card}; border: 1px solid {border}; border-radius: 10px }} \
         .accent {{ color: {accent} }} \
         .title {{ font-size: 44px; font-weight: bold; color: {accent}; margin: 60px 0 4px 0 }} \
         .subtitle {{ color: {muted}; margin: 0 0 32px 0 }}",
        bg = hex(palette.page_background),
        text = hex(palette.page_text),
        muted = hex(palette.page_muted),
        link = hex(palette.page_link),
        card = hex(palette.page_card),
        border = hex(palette.page_border),
        accent = hex(palette.accent),
    )
}
