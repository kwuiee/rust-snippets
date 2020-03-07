#[derive(Default, Debug)]
struct Target {
    total: u64,
    target: u64,
    flank: u64,
}

impl Target {
    fn total(&mut self) -> &mut u64 {
        &mut self.total
    }
}

fn main() {
    let mut s = Target::default();
    *s.total() += 1;
    println!("{:?}", s);
}
