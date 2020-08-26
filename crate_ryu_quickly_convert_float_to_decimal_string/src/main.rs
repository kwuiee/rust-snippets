fn main() {
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
