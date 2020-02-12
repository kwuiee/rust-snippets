extern crate rand;

use rand::prelude::*;

fn main() {
    if rand::random() {
        // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", rand::random::<char>());
    }

    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    println!("{}", y);

    let mut nums: Vec<i32> = (1..20).collect();
    nums.shuffle(&mut rng);
    println!("{:?}", nums);
}
