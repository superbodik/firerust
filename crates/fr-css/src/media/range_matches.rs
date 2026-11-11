use crate::media::length_to_px::length_to_px;
use crate::media::viewport::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub fn range_matches(text: &str) -> bool {
    let spaced = space_operators(text);
    let tokens: Vec<&str> = spaced.split_whitespace().collect();
    match tokens.as_slice() {
        [left, operator, right] => compare(left, operator, right),
        [first, first_operator, middle, last_operator, last] => {
            compare(first, first_operator, middle) && compare(middle, last_operator, last)
        }
        _ => false,
    }
}

fn space_operators(text: &str) -> String {
    let mut result = String::new();
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '<' | '>' => {
                result.push(' ');
                result.push(character);
                if characters.peek() == Some(&'=') {
                    result.push('=');
                    characters.next();
                }
                result.push(' ');
            }
            '=' => result.push_str(" = "),
            other => result.push(other),
        }
    }
    result
}

fn value_of(token: &str) -> Option<f32> {
    match token {
        "width" => Some(VIEWPORT_WIDTH),
        "height" => Some(VIEWPORT_HEIGHT),
        other => length_to_px(other),
    }
}

fn compare(left: &str, operator: &str, right: &str) -> bool {
    let (Some(left), Some(right)) = (value_of(left), value_of(right)) else {
        return false;
    };
    match operator {
        "<" => left < right,
        "<=" => left <= right,
        ">" => left > right,
        ">=" => left >= right,
        "=" => left == right,
        _ => false,
    }
}
