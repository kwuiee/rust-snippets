struct Histogram {
    sum: u32,
    bins: [u32; 256],
}

impl Default for Histogram {
    fn default() -> Histogram {
        unsafe { std::mem::zeroed() }
    }
}

fn main() {
    let s = Histogram::default();
    println!("{} {}", s.sum, s.bins[155]);
}
