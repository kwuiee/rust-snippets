use std::thread;
use std::time::Duration;

fn main() {
    let original_thread = thread::spawn(|| {
        let _detached_thread = thread::spawn(|| {
            // Here we sleep to make sure that the first thread returns before.
            thread::sleep(Duration::from_millis(10));
            // This will be called, even though the JoinHandle is dropped.
            println!("♫ Still alive ♫");
        });
    });
    original_thread
        .join()
        .expect("The thread being joined has panicked");
    println!("Original thread is joined.");

    // We make sure that the new thread has time to run, before the main
    // thread returns.

    thread::sleep(Duration::from_millis(1000));
}
