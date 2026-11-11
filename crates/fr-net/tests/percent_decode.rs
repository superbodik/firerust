use fr_net::address::percent_decode::percent_decode;

#[test]
fn escaped_bytes_are_turned_back_into_text() {
    assert_eq!(percent_decode("a%20b"), "a b");
    assert_eq!(
        percent_decode("%D0%BF%D1%80%D0%B8%D0%B2%D0%B5%D1%82"),
        "привет"
    );
}

#[test]
fn a_plus_stays_a_plus() {
    assert_eq!(percent_decode("1+1"), "1+1");
}

#[test]
fn a_broken_escape_is_kept_as_it_is() {
    assert_eq!(percent_decode("100%"), "100%");
    assert_eq!(percent_decode("%zz"), "%zz");
    assert_eq!(percent_decode("%4"), "%4");
}
