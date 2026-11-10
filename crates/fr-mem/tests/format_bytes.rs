use fr_mem::format::format_bytes::format_bytes;

#[test]
fn small_values_are_shown_in_bytes() {
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(1023), "1023 B");
}

#[test]
fn kilobytes_have_no_fraction() {
    assert_eq!(format_bytes(2048), "2 KB");
    assert_eq!(format_bytes(1536), "2 KB");
}

#[test]
fn megabytes_have_one_decimal() {
    assert_eq!(format_bytes(5 * 1024 * 1024 + 512 * 1024), "5.5 MB");
}

#[test]
fn gigabytes_have_one_decimal() {
    assert_eq!(format_bytes(3 * 1024 * 1024 * 1024), "3.0 GB");
}
