use async_process::{Command, Stdio};
use futures_lite::{future, io::BufReader, AsyncBufReadExt, StreamExt};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("find")
        .arg(".")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    future::block_on(async {
        while let Some(line) = lines.next().await {
            println!("{}", line?);
        }
        Ok::<(), std::io::Error>(())
    })?;
    Ok(())
}
