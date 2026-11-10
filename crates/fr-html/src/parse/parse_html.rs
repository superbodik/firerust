use fr_dom::tree::document::Document;
use html5ever::tendril::TendrilSink;
use html5ever::{ParseOpts, parse_document};
use markup5ever_rcdom::RcDom;

use crate::parse::convert_dom::convert_dom;

pub fn parse_html(html: &str) -> Document {
    let dom = parse_document(RcDom::default(), ParseOpts::default()).one(html);
    convert_dom(&dom)
}
