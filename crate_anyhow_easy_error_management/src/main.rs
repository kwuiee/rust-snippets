extern crate anyhow;
extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};

use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::str;
use std::sync::mpsc::channel;
use std::thread;
use toml::value::Value;

#[derive(Deserialize, Serialize)]
struct Config {
    username: String,
    id: u16,
}

fn main() -> Result<()> {
    let mut content = vec![];
    File::open("Cargo.toml")?.read_to_end(&mut content)?;
    let value: Value = toml::from_slice(&content)?;
    println!("{}\n", value["dependencies"]["serde"]);
    let val: Config = toml::from_str(&"username = 'Sean Lyo'\nid = 1234")?;
    println!("{}\n", val.username);
    let ser: String = toml::to_string_pretty(&val)?;
    println!("{}\n", ser);

    // threading error handling
    let (tx, rx) = channel();
    let sender = thread::spawn(move || -> Result<u8> {
        tx.send("Hello, thread".to_owned())?;
        Ok(0)
    });
    let receiver = thread::spawn(move || -> Result<u8> {
        let value = rx.recv()?;
        println!("{}", value);
        Ok(0)
    });
    match sender.join() {
        Ok(t) => println!("return value of send: {}", t?),
        Err(err) => println!("error returning send value: {:?}", err),
    };
    match receiver.join() {
        Ok(t) => println!("return value of send: {}", t?),
        Err(err) => println!("error returning send value: {:?}", err),
    };
    Ok(())
}
