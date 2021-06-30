use linya::{Bar, Progress};
use rayon::prelude::*;
use std::sync::Mutex;

fn main() {
    let progress = Mutex::new(Progress::new());

    // `into_par_iter()` is from `rayon`, and lets us parallelize some
    // operation over a collection "for free".
    (0..10).into_par_iter().for_each(|n| {
        let bar: Bar = progress
            .lock()
            .unwrap()
            .bar(50, format!("Downloading {}", n));

        // ... Your logic ...

        // Increment the bar and draw it immediately.
        // This is likely called in some inner loop or other closure.
        progress.lock().unwrap().inc_and_draw(&bar, 10);
    });
}
