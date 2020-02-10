extern crate csv;
extern crate serde;

use serde::Deserialize;
use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    region: String,
    #[serde(rename = "popcount")]
    population: Option<u64>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // 这里需要注意，我们需要为自动反序列化提供一个类型提示
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
