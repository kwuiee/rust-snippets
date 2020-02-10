extern crate tempfile;

use std::fs::OpenOptions;
use std::io::{Read, Result, Write};
use tempfile::tempfile;

fn main() -> Result<()> {
    // let mut temp = tempfile()?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("named.txt")?;
    // writeln!(temp, "Hello, world!")?;
    writeln!(file, "Hello, world!")?;
    let mut content = String::new();
    // temp.flush()?;
    file.flush()?;
    // temp.read_to_string(&mut content)?;
    file.read_to_string(&mut content)?;
    println!("{}", content);
    Ok(())
}
