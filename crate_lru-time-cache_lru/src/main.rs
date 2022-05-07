extern crate lru_time_cache;

use lru_time_cache::LruCache;

fn main() {
    // Construct an `LruCache` of `<u8, String>`s, limited by key count
    let max_count = 10;
    let _lru_cache = LruCache::<u8, String>::with_capacity(max_count);

    // Construct an `LruCache` of `<String, i64>`s, limited by expiry time
    let time_to_live = ::std::time::Duration::from_millis(100);
    let _lru_cache = LruCache::<String, i64>::with_expiry_duration(time_to_live);

    // Construct an `LruCache` of `<u64, Vec<u8>>`s, limited by key count and expiry time
    let _lru_cache =
        LruCache::<u64, Vec<u8>>::with_expiry_duration_and_capacity(time_to_live, max_count);
}
