extern crate humantime;

use std::time::{Duration, SystemTime};

fn main() {
    // Format
    // UTC to Beijing time.
    let time = SystemTime::now() + Duration::from_secs(8 * 3600);
    let timef = humantime::format_rfc3339(time);
    println!("{}", timef);
    // Parse
    assert_eq!(
        time,
        humantime::parse_rfc3339(&format!("{}", timef)).unwrap()
    );
}
