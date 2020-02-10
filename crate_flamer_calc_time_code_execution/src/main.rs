extern crate flame;

use std::fs::File;
use std::thread;
use std::time::Duration;

fn cpu_heavy_operations_1() {
    thread::sleep(Duration::from_secs(4))
}

fn cpu_heavy_operations_2() {
    thread::sleep(Duration::from_secs(3))
}

pub fn main() {
    // Manual `start` and `end`
    flame::start("read file");
    cpu_heavy_operations_1();
    cpu_heavy_operations_2();
    flame::end("read file");

    // Dump the report to disk
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

    // Or read and process the data yourself!
    let spans = flame::spans();

    println!("{} {} {}", x, y, z);
}
