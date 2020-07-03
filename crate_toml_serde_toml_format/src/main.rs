extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str;
use toml::value::Value;

#[derive(Deserialize, Serialize)]
struct Config {
    username: String,
    id: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    // deserialze as toml::value::Value
    let mut content = vec![];
    File::open("Cargo.toml")?.read_to_end(&mut content)?;
    let value: Value = toml::from_slice(&content)?; //str(&str::from_utf8(&content)?)?;
    println!("{}\n", value["dependencies"]["serde"]);

    // deserialize as local defined type
    let val: Config = toml::from_str(&"username = 'Sean Lyo'\nid = 1234")?;
    println!("{}\n", val.username);

    // serialize to string
    let ser: String = toml::to_string_pretty(&val)?;
    println!("{}\n", ser);
    Ok(())
}
