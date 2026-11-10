pub fn collapse_margins(first: f32, second: f32) -> f32 {
    if first >= 0.0 && second >= 0.0 {
        first.max(second)
    } else if first <= 0.0 && second <= 0.0 {
        first.min(second)
    } else {
        first + second
    }
}
