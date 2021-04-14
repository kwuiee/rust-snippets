use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

use ipipe::Pipe;

const CANCEL: u8 = 24;

fn main() {
    let mut pipe = Pipe::create().unwrap();
    println!("Name: {}", pipe.path().display());

    let writer = pipe.clone();
    let reader = pipe.clone();
    thread::spawn(move || print_nums(writer));
    thread::spawn(move || display_nums(reader));
    thread::sleep(Duration::from_secs(3));
    pipe.close().unwrap();
}

fn print_nums(mut pipe: Pipe) {
    for i in 1..=10 {
        writeln!(&mut pipe, "{}", i).unwrap();
    }
    write!(&mut pipe, "{}", CANCEL as char).unwrap();
}

fn display_nums(pipe: Pipe) {
    for line in BufReader::new(pipe).lines() {
        println!("{}", line.unwrap());
    };
}
