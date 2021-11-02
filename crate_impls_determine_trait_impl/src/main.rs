use impls::impls;

const IMPLS: bool = impls!(u8: From<u32>);

fn main() {
    println!("implemented: {}", IMPLS);

    // Precedence and Nesting
    let pre = impls!(u64:   From<u8> | From<u16>  ^ From<u32>  & From<u64>);
    let ltr = impls!(u64: ((From<u8> | From<u16>) ^ From<u32>) & From<u64>);

    assert_eq!(pre, true | true ^ true & true);
    assert_ne!(pre, ltr);

    // Reference Types
    assert!(impls!(&mut u32: !Copy & !Clone));
}
