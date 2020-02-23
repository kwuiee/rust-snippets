extern crate serde_json;

use std::collections::{BTreeMap, HashMap};

fn main() {
    let mut s = HashMap::new();
    let mut t = BTreeMap::new();
    s.insert("number", 1);
    t.insert("num", 1);
    println!("{}", serde_json::to_string_pretty(&s).unwrap());
    println!("{}", serde_json::to_string_pretty(&t).unwrap());
}
