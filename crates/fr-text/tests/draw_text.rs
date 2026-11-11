use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_raster::draw::clear::clear;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::draw_text::draw_text;
use fr_text::render::text_alignment::TextAlignment;
use fr_text::render::text_family::TextFamily;
use fr_text::render::text_run::TextRun;

const WHITE: Rgba = Rgba::opaque(255, 255, 255);

fn run(text: &str, color: Rgba) -> TextRun<'_> {
    TextRun {
        text,
        color,
        font_size: 20.0,
        line_height: 26.0,
        font_weight: 400,
        is_italic: false,
        family: TextFamily::SansSerif,
        is_underlined: false,
        is_struck: false,
    }
}

fn painted(engine: &mut TextEngine, text: &str, color: Rgba) -> Vec<u8> {
    let mut canvas = Canvas::new(200, 40).expect("canvas");
    clear(&mut canvas, WHITE);
    draw_text(
        engine,
        &mut canvas,
        &[run(text, color)],
        190.0,
        TextAlignment::Left,
        Point::new(4.0, 4.0),
    );
    canvas.pixmap().data().to_vec()
}

fn dark_pixels(data: &[u8]) -> usize {
    data.chunks(4)
        .filter(|pixel| pixel[0] < 200 || pixel[1] < 200 || pixel[2] < 200)
        .count()
}

#[test]
fn a_cached_draw_looks_exactly_like_the_first_one() {
    let mut engine = TextEngine::new();

    let first = painted(&mut engine, "Hello cached world", Rgba::opaque(0, 0, 0));
    let second = painted(&mut engine, "Hello cached world", Rgba::opaque(0, 0, 0));

    assert!(dark_pixels(&first) > 50);
    assert_eq!(first, second);
}

#[test]
fn the_colour_is_part_of_the_cache_key() {
    let mut engine = TextEngine::new();

    let black = painted(&mut engine, "Colour", Rgba::opaque(0, 0, 0));
    let red = painted(&mut engine, "Colour", Rgba::opaque(255, 0, 0));

    assert_ne!(black, red);
    assert!(red.chunks(4).any(|pixel| pixel[0] > 200 && pixel[1] < 80));
}

#[test]
fn different_text_is_not_mistaken_for_cached_text() {
    let mut engine = TextEngine::new();

    let short = painted(&mut engine, "ab", Rgba::opaque(0, 0, 0));
    let long = painted(&mut engine, "abababababab", Rgba::opaque(0, 0, 0));

    assert!(dark_pixels(&long) > dark_pixels(&short));
}

#[test]
fn many_different_lines_do_not_grow_the_cache_without_limit() {
    let mut engine = TextEngine::new();

    for number in 0..1_700 {
        let _ = painted(
            &mut engine,
            &format!("line {number}"),
            Rgba::opaque(0, 0, 0),
        );
    }

    let again = painted(&mut engine, "line 5", Rgba::opaque(0, 0, 0));
    assert!(dark_pixels(&again) > 20);
}
