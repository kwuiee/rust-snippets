extern crate bam;

mod bed;

use bam::bam_reader::IndexedReader;
use bam::pileup::Pileup;
use bam::Region;
use bed::Reader as BedReader;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

struct Depth {
    length: [u32; 400],
    above: u32,
    covered: u32,
    total: u32,
}

impl Depth {
    fn add_total(&mut self, v: u32) {
        self.total += v
    }

    fn add(&mut self, depth: usize) {
        if depth < self.length.len() {
            self.length[depth] += 1
        } else {
            self.above += 1
        }
        self.covered += (depth >= 1) as u32;
    }

    fn above(&self) -> u32 {
        self.above
    }

    fn perc_of(&self, depth: usize) -> f64 {
        if depth > self.length.len() {
            0f64
        } else {
            self.length[depth] as f64 / self.total as f64
        }
    }

    fn perc_above(&self) -> f64 {
        self.above as f64 / self.total as f64
    }

    fn len(&self) -> usize {
        self.length.len()
    }

    fn depth_of(&self, depth: usize) -> u32 {
        if depth > self.length.len() {
            0
        } else {
            self.length[depth]
        }
    }

    fn add_region_zero(&mut self, v: u32) {
        self.length[0] += v
    }

    fn inner_get_zero(&mut self) {
        println!("total {} covered {}", self.total, self.covered);
        self.length[0] = self.total - self.covered
    }
}

fn simple_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

fn run(input: &String, bed: &String) -> Result<(), Box<dyn Error>> {
    let mut reader = IndexedReader::from_path(input)?;
    let mut beder = BedReader::from_file(bed)?;
    let header = reader.header().to_owned();
    let mut depth = Depth {
        length: [0; 400],
        total: 0,
        above: 0,
        covered: 0,
    };
    while let Some(rec) = beder.records().next() {
        let rec = rec?;
        depth.add_total((rec.end() - rec.start() + 1) as u32);
        let ref_id = &mut header
            .reference_id(rec.chrom())
            .ok_or(simple_error(&format!(
                "chromsome {} not found",
                rec.chrom()
            )))?
            .to_owned();
        let reg = Region::new(*ref_id, rec.start() as u32, rec.end() as u32);
        for i in Pileup::new(&mut reader.fetch(&reg)?) {
            let pc = i?;
            if !reg.contains(pc.ref_id(), pc.ref_pos() as u32) {
                continue;
            }
            depth.add(pc.entries().len());
        }
    }
    depth.inner_get_zero();
    let mut outfile = File::create("freq.xls")?;
    for i in 0..depth.length.len() {
        outfile
            .write(format!("{}\t{}\t{}\n", i, depth.depth_of(i), depth.perc_of(i)).as_bytes())?;
    }
    Ok(())
}

fn main() {
    run(&env::args().nth(1).unwrap(), &env::args().nth(2).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn with_bed() -> Result<(), Box<dyn Error>> {
        run(
            &"/TJPROJ4/BioAI/liuxiaochuan/langs/dev/bsum/data/Sample_A.dedup.bam".to_owned(),
            &"tests/agilent_region.B37.bed".to_owned(),
        )
    }
}
