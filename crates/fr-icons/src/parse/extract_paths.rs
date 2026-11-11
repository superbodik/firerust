pub fn extract_paths(svg: &str) -> Vec<(String, bool)> {
    let mut found = Vec::new();
    let mut rest = svg;
    while let Some(start) = rest.find("<path") {
        let after = &rest[start..];
        let Some(end) = after.find('>') else {
            break;
        };
        let tag = &after[..end];
        if let Some(data) = attribute(tag, "d") {
            found.push((data.to_owned(), tag.contains("fill-rule=\"evenodd\"")));
        }
        rest = &after[end..];
    }
    found
}

fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!(" {name}=\"");
    let start = tag.find(&marker)? + marker.len();
    let length = tag[start..].find('"')?;
    Some(&tag[start..start + length])
}
