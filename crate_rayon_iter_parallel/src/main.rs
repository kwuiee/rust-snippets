extern crate rayon;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let s = vec![1, 2, 3, 4];
    ThreadPoolBuilder::new().num_threads(2).build_global()?;
    let t: Vec<i32> = s
        .par_iter()
        .map(|i| {
            thread::sleep(Duration::from_secs(2));
            println!("number: {}", i);
            i * i
        })
        .collect();
    println!("{:?}", t);
    Ok(())
}
