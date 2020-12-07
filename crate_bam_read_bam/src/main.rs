extern crate bam;

// You need to import RecordReader trait
use bam::index::Index;
use bam::{IndexedReader, Pileup, RecordReader, Region};

fn main() {
    // Read per record
    let mut reader = bam::BamReader::from_path("tests/sample.bam", 0).unwrap();
    let mut record = bam::Record::new();
    let mut corr = 0;
    loop {
        match reader.read_into(&mut record) {
            Ok(true) => {}
            Ok(false) => break,
            Err(e) => {
                corr += 1;
                println!("{}", e)
            }
        }
        // Do something.
    }
    println!("Corrupted number {}", corr);
    // Read index
    let index = Index::from_path("tests/sample.bam.bai").unwrap();
    for i in index.fetch_chunks(0, 127693, 386189) {
        println!("{}", i);
    }
    // Indexed pileup
    let mut ireader = IndexedReader::from_path("tests/sample.bam").unwrap();
    for pc in Pileup::new(&mut ireader.fetch(&Region::new(0, 127693, 386189)).unwrap()) {
        let pc = pc.unwrap();
        println!(
            "ref {} position {} depth {}",
            pc.ref_id(),
            pc.ref_pos(),
            pc.entries().len()
        )
    }
}
