extern crate rust_htslib;

use rust_htslib::bam::{IndexedReader, Read};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = env::args().nth(1).unwrap();
    let mut reader = IndexedReader::from_path(input)?;
    reader.fetch(0, 12080, 12251)?;
    for i in reader.pileup() {
        let i = i?;
        println!("ref {} pos {} depth {}", i.tid(), i.pos(), i.depth());
    }
    Ok(())
}
