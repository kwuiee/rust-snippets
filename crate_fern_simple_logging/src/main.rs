extern crate chrono;
extern crate fern;

use log::{debug, error, info, trace, warn};
use std::error::Error;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("test-data/output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;
    info!("info");
    debug!("debug");
    error!("error");
    warn!("warn");
    trace!("trace");
    Ok(())
}
