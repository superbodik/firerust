pub fn decode_percent(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => decoded.push(b' '),
            b'%' => match hex_pair(bytes, index + 1) {
                Some(value) => {
                    decoded.push(value);
                    index += 2;
                }
                None => decoded.push(b'%'),
            },
            other => decoded.push(other),
        }
        index += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn hex_pair(bytes: &[u8], start: usize) -> Option<u8> {
    let pair = bytes.get(start..start + 2)?;
    let text = std::str::from_utf8(pair).ok()?;
    u8::from_str_radix(text, 16).ok()
}
