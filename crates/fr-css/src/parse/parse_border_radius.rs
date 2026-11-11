use cssparser::Parser;

use crate::parse::declaration::Declaration;
use crate::parse::parse_length::parse_length;
use crate::value::corner::Corner;
use crate::value::length::Length;

const MAX_VALUES: usize = 4;

pub fn parse_border_radius(parser: &mut Parser<'_>) -> Option<Vec<Declaration>> {
    let mut values: Vec<Length> = Vec::new();
    while !parser.is_exhausted() {
        if parser.try_parse(|inner| inner.expect_delim('/')).is_ok() {
            skip_rest(parser);
            break;
        }
        values.push(parse_radius(parser)?);
        if values.len() > MAX_VALUES {
            return None;
        }
    }
    let [top_left, top_right, bottom_right, bottom_left] = match values.as_slice() {
        [all] => [*all; 4],
        [diagonal, anti] => [*diagonal, *anti, *diagonal, *anti],
        [first, anti, third] => [*first, *anti, *third, *anti],
        [first, second, third, fourth] => [*first, *second, *third, *fourth],
        _ => return None,
    };
    Some(vec![
        Declaration::BorderRadius(Corner::TopLeft, top_left),
        Declaration::BorderRadius(Corner::TopRight, top_right),
        Declaration::BorderRadius(Corner::BottomRight, bottom_right),
        Declaration::BorderRadius(Corner::BottomLeft, bottom_left),
    ])
}

pub fn parse_corner_radius(parser: &mut Parser<'_>, corner: Corner) -> Option<Vec<Declaration>> {
    let radius = parse_radius(parser)?;
    skip_rest(parser);
    Some(vec![Declaration::BorderRadius(corner, radius)])
}

fn parse_radius(parser: &mut Parser<'_>) -> Option<Length> {
    let length = parse_length(parser)?;
    match length {
        Length::Auto => None,
        Length::Px(value) | Length::Em(value) | Length::Rem(value) | Length::Percent(value)
            if value < 0.0 =>
        {
            None
        }
        other => Some(other),
    }
}

fn skip_rest(parser: &mut Parser<'_>) {
    while parser.next().is_ok() {}
}
