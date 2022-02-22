use std::fs;
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut f = BufWriter::new(fs::File::create("/tmp/buffered.txt")?);
    f.write(b"foo")?;
    f.write(b"\n")?;
    f.write(b"bar\nbaz\n")?;
    return Ok(());
}
