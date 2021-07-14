use yore::code_pages::{CP850, CP857};
use yore::{DecodeError, EncodeError};

fn main() {
    // Vec contains ascii "text"
    let bytes = vec![116, 101, 120, 116];
    // Vec contains ascii "text " and codepoint 231
    let bytes_undefined = vec![116, 101, 120, 116, 32, 231];

    // notice that decoding CP850 can't fail because it is completely defined
    assert_eq!(CP850.decode(&bytes), "text");

    // But CP857 can fail
    assert_eq!(CP857.decode(&bytes).unwrap(), "text");

    //"text " + codepoint 231
    assert!(matches!(CP857.decode(&bytes_undefined), DecodeError));

    // lossy decoding will not fail because of fallback
    assert_eq!(CP857.decode_lossy(&bytes_undefined), "text �");

    // encoding
    assert_eq!(CP850.encode("text").unwrap(), bytes);
    assert!(matches!(CP850.encode("text 🦀"), EncodeError));
    assert_eq!(CP850.encode_lossy("text 🦀", 231), bytes_undefined);
}
