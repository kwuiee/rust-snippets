#[async_std::main]
async fn main() -> heim::Result<()> {
    let platform = heim::host::platform().await?;

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
