extern crate zip;

use std::error::Error;

fn doit() -> Result<(), Box<dyn Error>> {
    // For demonstration purposes we read from an empty buffer.
    // Normally a File object would be used.
    let reader = std::fs::File::open("tests/empty.zip")?;
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let file = zip.by_index(i).unwrap();
        println!("Filename: {}", file.name());
    }
    Ok(())
}

fn main() {
    println!("Result: {:?}", doit());
}
