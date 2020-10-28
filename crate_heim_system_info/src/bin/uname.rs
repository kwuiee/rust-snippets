//! `uname -a` implementation

use heim::{host, Result};

#[smol_potat::main]
async fn main() -> Result<()> {
    let platform = host::platform().await?;

    println!(
        "{} {} {} {} {}",
        platform.system(),
        platform.release(),
        platform.hostname(),
        platform.version(),
        platform.architecture().as_str(),
    );

    Ok(())
}
