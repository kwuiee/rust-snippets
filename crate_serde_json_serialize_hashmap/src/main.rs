extern crate serde_json;

use std::collections::HashMap;

fn main() {
    let mut s = HashMap::new();
    s.insert("number", 1);
    println!("{}", serde_json::to_string_pretty(&s).unwrap());
}
