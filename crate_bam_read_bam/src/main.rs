extern crate bam;

// You need to import RecordReader trait
use bam::RecordReader;
use std::env;

fn main() {
    let mut reader = bam::BamReader::from_path(env::args().nth(1).unwrap(), 4).unwrap();
    let mut record = bam::Record::new();
    let mut corru = 1;
    loop {
        match reader.read_into(&mut record) {
            Ok(()) => {}
            Err(bam::Error::NoMoreRecords) => break,
            Err(e) => {
                corru += 1;
                println!("{}", e)
            }
        }
        // Do something.
    }
    println!("corrupted number: {}", corru)
}
