//!
//! [Ref](https://rust-china.org/article?id=c7ff2472-b990-4b63-93ab-6ea16009ebe0)
//!
#[macro_use]
extern crate failure;

use std::io;
use std::io::BufRead;

use failure::err_msg;
use failure::Error;

fn my_function() -> Result<(), Error> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;

        if line.chars().all(|c| c.is_whitespace()) {
            break;
        }

        if !line.starts_with("$") {
            return Err(format_err!("Input did not begin with `$`"));
        }

        println!("{}", &line[1..]);
    }

    Ok(())
}

fn main() {}
