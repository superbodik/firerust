use tiny_skia::Pixmap;

pub fn copy_into_xrgb(pixmap: &Pixmap, target: &mut [u32]) {
    for (target_pixel, source) in target.iter_mut().zip(pixmap.data().chunks_exact(4)) {
        let red = u32::from(source[0]);
        let green = u32::from(source[1]);
        let blue = u32::from(source[2]);
        *target_pixel = (red << 16) | (green << 8) | blue;
    }
}
