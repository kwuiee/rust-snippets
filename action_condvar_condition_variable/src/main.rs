use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    let sender = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        thread::sleep(std::time::Duration::from_secs(1));
        let mut started = lock.lock().unwrap();
        *started = true;
        thread::sleep(std::time::Duration::from_secs(5));
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    let receiver = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            // Any calls to notify_one or notify_all which happen logically
            // after the mutex is unlocked are candidates to wake this thread up.
            started = cvar.wait(started).unwrap();
        }
    });
    sender.join().unwrap();
    receiver.join().unwrap();
}
