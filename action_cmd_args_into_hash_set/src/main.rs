use std::collections::HashSet;
use std::env;

fn main() {
    let args: HashSet<String> = env::args().collect();
    let verbose_flag = args.contains("--verbose");
    if verbose_flag {
        println!("verbose contains: {}", verbose_flag)
    }
}
