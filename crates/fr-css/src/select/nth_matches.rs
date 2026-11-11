pub fn nth_matches(step: i32, offset: i32, index: i32) -> bool {
    if step == 0 {
        return index == offset;
    }
    let distance = index - offset;
    distance % step == 0 && distance / step >= 0
}
