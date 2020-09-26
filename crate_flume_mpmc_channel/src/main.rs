use std::thread;

fn main() {
    let (tx, rx) = flume::unbounded();

    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i);
        })
    });

    let received: i32 = rx.iter().sum();

    assert_eq!((0..10).sum::<i32>(), received);
}
