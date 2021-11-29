extern crate signal_hook;

use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        // Do some time-limited stuff here
        // (if this could block forever, then there's no guarantee the signal will have any
        // effect).
        println!("Sleep 1");
        thread::sleep(Duration::from_secs(1));
    }
    println!("exiting ...");
    thread::sleep(Duration::from_secs(5));
    Ok(())
}
