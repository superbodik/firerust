const KILOBYTE: f64 = 1024.0;
const MEGABYTE: f64 = 1024.0 * 1024.0;
const GIGABYTE: f64 = 1024.0 * 1024.0 * 1024.0;

pub fn format_bytes(bytes: u64) -> String {
    let value = bytes as f64;
    if value >= GIGABYTE {
        format!("{:.1} GB", value / GIGABYTE)
    } else if value >= MEGABYTE {
        format!("{:.1} MB", value / MEGABYTE)
    } else if value >= KILOBYTE {
        format!("{:.0} KB", value / KILOBYTE)
    } else {
        format!("{bytes} B")
    }
}
