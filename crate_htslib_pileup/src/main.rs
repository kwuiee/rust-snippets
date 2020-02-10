extern crate rust_htslib;

use rust_htslib::bam::{errors::Result, IndexedReader, Read};
use std::env;

fn main() -> Result<()> {
    let bam: &str = &env::args().nth(1).unwrap();
    let mut reader = IndexedReader::from_path_and_index(bam, &(bam.to_owned() + ".bai"))?;
    reader.fetch(0, 127693, 386189)?;
    for i in reader.pileup() {
        let item = i?;
        println!(
            "chr: {} position: {} depth: {}",
            item.tid(),
            item.pos(),
            item.depth()
        );
    }
    Ok(())
}
