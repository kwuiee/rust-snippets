extern crate rug;

use rug::ops::Pow;
use rug::Integer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let power: u32 = 1000;
    let base = Integer::from_str_radix("ffff0000ffff0000ffff0000ffff0000ffff0000", 16)?;
    let basen = base.clone();
    println!(
        "The number {} power {} is {}",
        basen,
        power,
        base.pow(&power)
    );
    Ok(())
}
