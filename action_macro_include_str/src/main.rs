fn main() {
    let s = include_str!("hello.txt");
    assert_eq!(s, "hello\n");
}
