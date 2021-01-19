fn main() {
    // Parse the entire string as a decimal number.
    let s = "1.23e-02";
    let x: f32 = fast_float::parse(s).unwrap();
    assert_eq!(x, 0.0123);

    // Parse as many characters as possible as a decimal number.
    let s = "1.23e-02foo";
    let (x, n) = fast_float::parse_partial::<f32, _>(s).unwrap();
    assert_eq!(x, 0.0123);
    assert_eq!(n, 8);
    assert_eq!(&s[n..], "foo");
}
