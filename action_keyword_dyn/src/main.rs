extern crate flate2;

use flate2::read::MultiGzDecoder;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = env::args().nth(1).unwrap();
    let stream: Box<dyn BufRead> = match input.split('.').last() {
        Some("gz") => Box::new(BufReader::new(MultiGzDecoder::new(
            File::open(input).unwrap(),
        ))),
        Some(_) => Box::new(BufReader::new(File::open(input).unwrap())),
        None => panic!("unkown input file"),
    };
    let mut total_bytes: usize = 0;
    let mut line_number: u16 = 0;
    for i in stream.lines() {
        let sec = match i {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        };
        total_bytes += sec.len();
        line_number += 1;
    }
    println!("Summary: ln {} bytes {}", line_number, total_bytes);
}
