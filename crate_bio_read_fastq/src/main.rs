extern crate bio;
extern crate flate2;

use bio::io::fastq;
use flate2::read::MultiGzDecoder;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

fn main() -> Result<()> {
    let input = File::open(env::args().nth(1).unwrap())?;
    let stream = MultiGzDecoder::new(input);
    fastq::Reader::new(stream);
    Ok(())
}
