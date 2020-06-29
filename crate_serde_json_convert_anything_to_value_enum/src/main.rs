extern crate serde;
extern crate serde_json;

use serde::Serialize;
use serde_json::{json, to_value, Value};
use std::collections::BTreeMap;
use std::error::Error;

fn raise() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, "option error")
}

#[derive(Serialize)]
struct User {
    name: String,
    id: u16,
    location: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let u = User {
        name: String::from("John Doe"),
        id: 12345,
        location: "Iowa".to_owned(),
    };
    let v = to_value([1, 2, 3, 4])?;
    assert!(v.is_array());
    println!("value is {}", v.get(2).ok_or_else(raise)?);
    let j = json!({"A": 10.0, "B": 11.0, "C": 12.0});
    assert!(j.is_object());
    let mut store: BTreeMap<String, Value> = BTreeMap::new();
    store.insert("a".to_owned(), to_value(u)?);
    store.insert("b".to_owned(), v);
    store.insert("c".to_owned(), j);
    println!("example {}", store.get("a").ok_or_else(raise)?);
    Ok(())
}
