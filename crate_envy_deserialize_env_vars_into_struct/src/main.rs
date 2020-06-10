extern crate envy;
extern crate serde;

use serde::Deserialize;
use std::error::Error;
use std::result::Result;

#[derive(Deserialize, Debug)]
struct Config {
    // required, `FOO` variable
    foo: u16,
    // required, `BAR` variable
    bar: bool,
    // required, `HOME` variable
    home: String,
    // optional, `BOOM` variable
    boom: Option<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    match envy::from_env::<Config>() {
        Ok(config) => println!("{:#?}", config),
        Err(error) => eprintln!("{:#?}", error),
    }
    Ok(())
}
