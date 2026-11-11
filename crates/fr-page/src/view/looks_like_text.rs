const SAMPLE_BYTES: usize = 1024;

pub fn looks_like_text(body: &[u8]) -> bool {
    let sample = &body[..body.len().min(SAMPLE_BYTES)];
    !sample.contains(&0)
}
