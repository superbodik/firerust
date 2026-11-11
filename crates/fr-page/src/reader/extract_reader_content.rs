use fr_dom::tree::document::Document;

use crate::reader::html_text_length::html_text_length;
use crate::reader::pick_container::pick_container;
use crate::reader::reader_content::ReaderContent;
use crate::reader::reader_title::reader_title;
use crate::reader::serialize_clean::serialize_clean;

const MIN_READABLE: usize = 200;

pub fn extract_reader_content(document: &Document) -> Option<ReaderContent> {
    let container = pick_container(document)?;
    let html = serialize_clean(document, container);
    if html_text_length(&html) < MIN_READABLE {
        return None;
    }
    Some(ReaderContent {
        title: reader_title(document),
        html,
    })
}
