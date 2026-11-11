use crate::media::viewport::FONT_SIZE;

pub fn length_to_px(text: &str) -> Option<f32> {
    let trimmed = text.trim();
    let split = trimmed
        .find(|character: char| {
            !(character.is_ascii_digit() || matches!(character, '.' | '-' | '+'))
        })
        .unwrap_or(trimmed.len());
    let (number, unit) = trimmed.split_at(split);
    let value: f32 = number.parse().ok()?;
    match unit.trim() {
        "px" => Some(value),
        "em" | "rem" => Some(value * FONT_SIZE),
        "pt" => Some(value * 4.0 / 3.0),
        "" if value == 0.0 => Some(0.0),
        _ => None,
    }
}
