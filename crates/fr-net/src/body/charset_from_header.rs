use encoding_rs::Encoding;

pub fn charset_from_header(content_type: Option<&str>) -> Option<&'static Encoding> {
    let header = content_type?;
    let lowered = header.to_ascii_lowercase();
    let start = lowered.find("charset=")? + "charset=".len();
    let label: String = header[start..]
        .trim_start()
        .trim_start_matches(['"', '\''])
        .chars()
        .take_while(|character| !matches!(character, ';' | '"' | '\'' | ' ' | ','))
        .collect();
    Encoding::for_label(label.as_bytes())
}
