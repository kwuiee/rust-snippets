extern crate libc;
extern crate signal_hook;

use std::io::Error;

use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

fn main() -> Result<(), Error> {
    let mut signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
    'outer: loop {
        // Pick up signals that arrived since last time
        for signal in signals.pending() {
            match signal as libc::c_int {
                SIGHUP => {
                    // Reload configuration
                    // Reopen the log file
                    println!("Reloading cfg...");
                }
                SIGTERM | SIGINT | SIGQUIT => {
                    break 'outer;
                }
                _ => unreachable!(),
            }
        }
        // Do some bit of work ‒ something with upper limit on waiting, so we don't block
        // forever with a SIGTERM already waiting.
    }
    println!("Terminating. Bye bye");
    Ok(())
}
