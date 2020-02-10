extern crate notify;

use crossbeam_channel::unbounded;
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::time::Duration;

fn main() -> Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = unbounded();

    // Automatically select the best implementation for your platform.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("data/", RecursiveMode::NonRecursive)?;

    loop {
        match rx.recv().unwrap() {
            Ok(evt) => {
                println!("event kind: {:?}", evt.kind);
                println!("event paths: {:?}", evt.paths);
                println!("event attrs: {:?}", evt.attrs);
            }
            Err(err) => println!("watch error: {:?}", err),
        };
    }
    Ok(())
}
