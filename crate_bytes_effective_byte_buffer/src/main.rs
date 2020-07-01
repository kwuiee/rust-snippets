use bytes::{BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(64);
    // put &[u8]
    buf.put(&b"hello world"[..]);
    // u16
    buf.put_u16(1234);
    // move bytes
    let a = buf.split();
    assert_eq!(a, b"hello world\x04\xD2"[..]);
    buf.put(&b"goodby world"[..]);
    println!("length: {}", buf.len());
    // shorten buffer
    buf.truncate(5);
    assert_eq!(buf, b"goodb"[..]);
    // clear buffer
    buf.clear();
    assert_eq!(buf, b""[..]);
}
