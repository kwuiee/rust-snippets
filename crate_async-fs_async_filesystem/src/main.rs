use async_fs::File;
use futures_lite::future::block_on;
use futures_lite::io::AsyncWriteExt;
use std::io::Error;

fn main() -> Result<(), Error> {
    block_on(async {
        let mut file = File::create("/tmp/a.txt").await?;
        file.write_all(b"Hello, world!").await?;
        file.flush().await?;
        Ok::<(), Error>(())
    })?;
    Ok(())
}
