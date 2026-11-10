pub fn distribute_columns(
    min: &[f32],
    max: &[f32],
    target: f32,
    grows_to_target: bool,
) -> Vec<f32> {
    let total_min: f32 = min.iter().sum();
    let total_max: f32 = max.iter().sum();

    if target >= total_max {
        if !grows_to_target || total_max <= 0.0 {
            return max.to_vec();
        }
        let scale = target / total_max;
        return max.iter().map(|width| width * scale).collect();
    }
    if target >= total_min {
        let range = total_max - total_min;
        let share = if range > 0.0 {
            (target - total_min) / range
        } else {
            0.0
        };
        return min
            .iter()
            .zip(max)
            .map(|(low, high)| low + (high - low) * share)
            .collect();
    }
    if total_min <= 0.0 {
        return min.to_vec();
    }
    let scale = target.max(0.0) / total_min;
    min.iter().map(|width| width * scale).collect()
}
