extern crate bio;

use bio::io::bed;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let example = b"1\t5\t5000\n2\t45\t2342342\n";
    let mut reader = bed::Reader::new(&example[..]);
    let mut writer = bed::Writer::new(vec![]);
    for record in reader.records() {
        let rec = record?;
        println!("{}", rec.chrom());
        writer.write(&rec)?;
    }
    Ok(())
}
