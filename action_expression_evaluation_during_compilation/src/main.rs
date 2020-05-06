use std::collections::HashMap;

//!
//! [ref](https://doc.rust-lang.org/reference/const_eval.html)
//! updated will always be `true`
//!
fn main() {
    let mut map = HashMap::new();
    map.insert("key", 2);
    let mut updated = false;
    map.entry("key").or_insert({
        updated = true;
        1
    });
    println!("updated: {}", updated);
}
