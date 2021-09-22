use memuse::DynamicUsage;

fn main() {
    assert_eq!(7u64.dynamic_usage(), 0);
    assert_eq!("I'm simple!".dynamic_usage(), 0);
    assert_eq!(vec![7u64; 2].dynamic_usage(), 16);

    let empty: Vec<u32> = Vec::with_capacity(100);
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.dynamic_usage(), 400);
}
