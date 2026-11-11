use fr_net::address::percent_decode::percent_decode;
use fr_net::load::response::Response;

use crate::name::extension_for::extension_for;
use crate::name::parse_content_disposition::parse_content_disposition;
use crate::name::sanitize_file_name::sanitize_file_name;

pub fn file_name_for(response: &Response) -> String {
    let from_header = response
        .content_disposition
        .as_deref()
        .and_then(|header| parse_content_disposition(header).file_name);
    let from_address = || {
        response
            .url
            .path_segments()
            .and_then(|mut segments| segments.next_back().map(percent_decode))
            .filter(|segment| !segment.is_empty())
    };
    let raw = from_header.or_else(from_address).unwrap_or_default();
    let name = sanitize_file_name(&raw);
    if name.contains('.') {
        return name;
    }
    match response.content_type.as_deref().and_then(extension_for) {
        Some(extension) => format!("{name}.{extension}"),
        None => name,
    }
}
