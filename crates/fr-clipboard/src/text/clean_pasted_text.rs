const MAX_CHARACTERS: usize = 10_000;

pub fn clean_pasted_text(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '\n' | '\r' | '\t' => ' ',
            other => other,
        })
        .filter(|character| !character.is_control())
        .take(MAX_CHARACTERS)
        .collect()
}
