extern crate bam;

use bam::{pileup::Pileup, IndexedReader, Region};
use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let bamfile = env::args().nth(1).unwrap();
    let mut ireader = IndexedReader::from_path(bamfile)?;
    for pc in Pileup::new(&mut ireader.fetch(&Region::new(0, 127693, 386189))?) {
        let pc = pc?;
        println!(
            "ref {} position {} depth {}",
            pc.ref_id(),
            pc.ref_pos(),
            pc.entries().len()
        )
    }
    Ok(())
}
