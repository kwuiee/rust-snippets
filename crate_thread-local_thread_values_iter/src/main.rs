use std::cell::Cell;
use std::sync::Arc;
use std::thread;
use thread_local::ThreadLocal;

fn main() {
    let tls = Arc::new(ThreadLocal::new());

    // Create a bunch of threads to do stuff
    for _ in 0..5 {
        let tls2 = tls.clone();
        thread::spawn(move || {
            // Increment a counter to count some event...
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        })
        .join()
        .unwrap();
    }

    // Once all threads are done, collect the counter values and return the
    // sum of all thread-local counter values.
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| x + y.get());
    assert_eq!(total, 5);
}
