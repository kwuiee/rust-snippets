use blocking::{unblock, Unblock};
use futures_lite::*;
use std::fs::File;
use std::io::Error;
use std::io::Result;

fn main() -> Result<()> {
    future::block_on(async {
        let input: File = unblock(|| File::open("Cargo.toml")).await?;
        let input = Unblock::new(input);
        let mut output = Unblock::new(std::io::stdout());

        io::copy(input, &mut output).await?;
        Ok::<(), Error>(())
    })?;
    Ok(())
}
