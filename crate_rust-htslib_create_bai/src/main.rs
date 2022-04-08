extern crate rust_htslib;

use rust_htslib::bam::index;

fn main() {
    let bampath = std::env::args().nth(1).expect("Expected input bam file");
    index::build(
        &bampath,
        Some(&[&bampath, ".bai"].concat()),
        index::Type::BAI,
        2,
    )
    .unwrap();
}
