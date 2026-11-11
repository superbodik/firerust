use crate::media::length_to_px::length_to_px;
use crate::media::viewport::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub fn feature_matches(name: &str, value: &str) -> bool {
    let value = value.trim();
    let length = || length_to_px(value);
    match name {
        "min-width" | "min-device-width" => length().is_some_and(|px| VIEWPORT_WIDTH >= px),
        "max-width" | "max-device-width" => length().is_some_and(|px| VIEWPORT_WIDTH <= px),
        "width" | "device-width" => length().is_some_and(|px| VIEWPORT_WIDTH == px),
        "min-height" | "min-device-height" => length().is_some_and(|px| VIEWPORT_HEIGHT >= px),
        "max-height" | "max-device-height" => length().is_some_and(|px| VIEWPORT_HEIGHT <= px),
        "height" | "device-height" => length().is_some_and(|px| VIEWPORT_HEIGHT == px),
        "orientation" => value == "landscape",
        "prefers-color-scheme" => value == "light",
        "prefers-reduced-motion" => value == "no-preference",
        "prefers-contrast" => value == "no-preference",
        "prefers-reduced-transparency" => value == "no-preference",
        "forced-colors" | "inverted-colors" => value == "none",
        "hover" | "any-hover" => value == "hover",
        "pointer" | "any-pointer" => value == "fine",
        "scripting" => value == "enabled",
        "update" => value == "fast",
        "display-mode" => value == "browser",
        "min-resolution" | "-webkit-min-device-pixel-ratio" | "min-device-pixel-ratio" => {
            ratio(value).is_some_and(|number| 1.0 >= number)
        }
        "max-resolution" | "-webkit-max-device-pixel-ratio" | "max-device-pixel-ratio" => {
            ratio(value).is_some_and(|number| 1.0 <= number)
        }
        "resolution" | "-webkit-device-pixel-ratio" => {
            ratio(value).is_some_and(|number| number == 1.0)
        }
        _ => false,
    }
}

fn ratio(value: &str) -> Option<f32> {
    let digits = value
        .trim_end_matches("dppx")
        .trim_end_matches('x')
        .trim_end_matches("dpi")
        .trim();
    let number: f32 = digits.parse().ok()?;
    if value.ends_with("dpi") {
        return Some(number / 96.0);
    }
    Some(number)
}
