extern crate bam;

use bam::index::Index;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let index = Index::from_path("tests/Sample_A.dedup.bam.bai")?;
    for i in index.fetch_chunks(0, 11919, 13052) {
        println!("{}", i);
    }
    Ok(())
}
