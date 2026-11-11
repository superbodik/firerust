pub fn html_text_length(html: &str) -> usize {
    let mut count = 0;
    let mut inside_tag = false;
    for character in html.chars() {
        match character {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            other if !inside_tag && !other.is_whitespace() => count += 1,
            _ => {}
        }
    }
    count
}
