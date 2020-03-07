extern crate bam;

use bam::{header::Header, BamReader};
use std::fs::File;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let input: &str = &std::env::args().nth(1).unwrap();
    let bam: BamReader<File> = BamReader::from_path(input, 0)?;
    let header: &Header = bam.header();
    for (i, j) in header
        .reference_names()
        .iter()
        .zip(header.reference_lengths().iter())
    {
        println!(
            "refid {} ref {} len {}",
            header.reference_id(i).unwrap(),
            i,
            j
        )
    }
    Ok(())
}
