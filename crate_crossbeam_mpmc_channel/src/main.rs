extern crate crossbeam;
#[macro_use]
extern crate crossbeam_channel as channel;

use channel::{Receiver, Sender};
use std::any::Any;

fn seek<'a>(name: &'a str, tx: &Sender<&'a str>, rx: &Receiver<&'a str>) {
    select! {
        recv(rx) -> peer => {
            if let Ok(v) = peer {
                println!("{} received a message from {}.", name, v)
            };
        },
        send(tx, name) -> _ => {
        },
    }
}

fn main() -> Result<(), Box<dyn Any + 'static + Send>> {
    let people = vec!["Zhiming", "Xiaochuan", "Panpan", "Wang Ze", "Tao Wei"];
    let (tx, rx) = channel::bounded(1);
    let (tx, rx) = (&tx, &rx);

    crossbeam::scope(|s| {
        for name in people {
            s.spawn(move |_| seek(name, tx, rx));
        }
    })?;

    if let Ok(name) = rx.try_recv() {
        println!("No one received {}'s message.", name);
    };
    Ok(())
}
