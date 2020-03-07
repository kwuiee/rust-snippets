use std::env;
use std::mem::drop;
use std::ops::Drop;

#[derive(Debug)]
struct D(&'static str);

impl Drop for D {
    fn drop(&mut self) {
        println!("destructing: {}", self.0);
    }
}

fn condition() -> Option<u32> {
    env::var("DROP").map(|s| s.parse::<u32>().unwrap_or(0)).ok()
}

fn main() {
    let var = (D("first"), D("second"), D("third"));
    match condition() {
        Some(1) => drop(var.0),
        Some(2) => drop(var.1),
        _ => {}
    }
    println!("{:?}", var.2);
}
