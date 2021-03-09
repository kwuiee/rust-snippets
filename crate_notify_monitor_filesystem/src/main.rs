use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};

fn main() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(".", RecursiveMode::Recursive)?;

    loop {}
}
