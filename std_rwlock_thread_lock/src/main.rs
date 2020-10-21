#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;
use std::thread;

lazy_static! {
    pub static ref INC: RwLock<i32> = RwLock::new(0);
}

fn main() {
    let thread1 = thread::spawn(|| {
        for _ in 1..10 {
            let mut w = INC.write().unwrap();
            //let mut w = INC.write();
            *w = *w + 1;
        }
    });

    let thread2 = thread::spawn(|| {
        for _ in 1..10 {
            let mut w = INC.write().unwrap();
            //let mut w = INC.write();
            *w = *w + 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    let r = INC.read().unwrap();
    //let r = INC.read();
    println!("{}", *r); // 输出 18
}
