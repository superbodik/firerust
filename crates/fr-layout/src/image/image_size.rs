pub fn image_size(
    intrinsic: (f32, f32),
    specified_width: Option<f32>,
    specified_height: Option<f32>,
    max_width: Option<f32>,
    available: f32,
) -> (f32, f32) {
    let (natural_width, natural_height) = intrinsic;
    let ratio = if natural_width > 0.0 {
        natural_height / natural_width
    } else {
        1.0
    };
    let (mut width, mut height) = match (specified_width, specified_height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (width, width * ratio),
        (None, Some(height)) => (if ratio > 0.0 { height / ratio } else { 0.0 }, height),
        (None, None) => (natural_width, natural_height),
    };
    let limit = max_width.map_or(available, |maximum| maximum.min(available));
    if width > limit && width > 0.0 {
        height *= limit / width;
        width = limit;
    }
    (width.max(0.0), height.max(0.0))
}
