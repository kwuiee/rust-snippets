use rxrust::{
    ops::{Filter, Fork, Merge},
    prelude::*,
};

fn main() {
    let mut numbers = observable::from_iter(0..10);
    // crate a even stream by filter
    let even = numbers.fork().filter(|v| v % 2 == 0);
    // crate an odd stream by filter
    let odd = numbers.fork().filter(|v| v % 2 != 0);

    // merge odd and even stream again
    even.merge(odd).subscribe(|v| print!("{} ", v,));
    // "0 1 2 3 4 5 6 7 8 9" will be printed.
}
