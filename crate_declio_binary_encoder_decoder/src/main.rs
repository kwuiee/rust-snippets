use declio::ctx::Endian;
use declio::{Decode, Encode};
use std::convert::TryInto;

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithLength {
    // Context can be passed to the field decoder with a `ctx` attribute.
    #[declio(ctx = "Endian::Little")]
    len: u16,

    // Context may be different for encode and decode,
    // though they should generally be as symmetric as possible.
    // For example, `Vec` doesn't accept a `Len` when encoding.
    //
    // Fields declared before this one can be accessed by name
    // (or by `field_0`, `field_1`, etc for tuple structs):
    #[declio(ctx(
        encode = "Endian::Little",
        decode = "(len.try_into()?, Endian::Little)"
    ))]
    bytes: Vec<u8>,
}

fn main() {
    let bytes: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];

    let with_length = WithLength {
        len: bytes.len().try_into().expect("length out of range"),
        bytes,
    };

    let mut encoded: Vec<u8> = Vec::new();
    with_length.encode((), &mut encoded).expect("encode failed");
    assert_eq!(encoded, [0x04, 0x00, 0xde, 0xad, 0xbe, 0xef]);

    let mut decode_reader: &[u8] = encoded.as_slice();
    let decoded = WithLength::decode((), &mut decode_reader).expect("decode failed");

    assert!(decode_reader.is_empty());
    assert_eq!(decoded, with_length);
}
