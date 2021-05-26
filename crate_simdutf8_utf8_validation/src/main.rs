use simdutf8::{basic, compat};

fn main() {
    // Use simdutf8::basic::from_utf8() as a drop-in replacement for std::str::from_utf8().
    println!(
        "{}",
        basic::from_utf8(b"I \xE2\x9D\xA4\xEF\xB8\x8F UTF-8!").unwrap()
    );

    // If you need detailed information on validation failures, use simdutf8::compat::from_utf8()
    // instead.
    let err = compat::from_utf8(b"I \xE2\x9D\xA4\xEF\xB8 UTF-8!").unwrap_err();
    assert_eq!(err.valid_up_to(), 5);
    assert_eq!(err.error_len(), Some(2));
}
