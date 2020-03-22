extern crate rust_htslib;

use rust_htslib::bcf::{self, Read};
use std::error::Error;
use std::{env, io};

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn gerr<E>(error: E) -> io::Error
where
    E: Into<Box<dyn Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, error)
}

fn main() -> Result<(), Box<dyn Error>> {
    let vcfpath = env::args()
        .nth(1)
        .ok_or_else(|| gerr("argument required"))?;
    let mut vcf = bcf::Reader::from_path(vcfpath)?;
    for i in vcf.records() {
        let mut i = i?;
        if i.pos() == 39687954 {
            println!("DP4: {:?}", i.info(b"DP4").integer()?);
        };
    }
    Ok(())
}
