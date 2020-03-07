use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

const COUNT: u32 = 1000000;

fn main() -> Result<(), Box<dyn Error>> {
    let global = Arc::new(Mutex::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        println!("increasing");
        for _ in 0..COUNT {
            let mut value = clone1.lock().unwrap();
            *value += 1;
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        println!("decreasing");
        for _ in 0..COUNT {
            let mut value = clone2.lock().unwrap();
            *value -= 1;
        }
    });

    thread1.join().ok();
    thread2.join().ok();
    println!("final value: {:?}", global);
    Ok(())
}
