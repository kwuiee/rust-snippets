use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });
    while let Ok(r) = rx.recv() {
        println!("received {}", r)
    }
}
