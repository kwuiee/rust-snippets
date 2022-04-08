/*
 * sam/bam file summary with upper base number threshold ~1000G
 *
 * @param region: bed file
 * @param flank_length: default 200
 */
#![feature(rustc_private)]
extern crate clap;
extern crate rust_htslib;
extern crate serde;
extern crate serde_json;

use clap::{App, AppSettings};
use rust_htslib::bam::Reader as BamReader;
use rust_htslib::bam::{record::Record, Read};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize, Default)]
struct Target {
    total_reads: u32,
    paired_reads: u32,
    failed_qc_reads: u32,
    duplicate_reads: u32,
    mapped_reads: u32,
    // mapped_reads_25: u32,
    properly_mapped_reads: u32,
    pe_mapped_reads: u32,
    se_mapped_reads: u32,
    mate_on_diff_chr: u32,
    mate_on_diff_chr_q5: u32,
    first_reads: u32,
    second_reads: u32,
    forward_strand_reads: u32,
    backward_strand_reads: u32,
    secondary_reads: u32,
    supplymentary_reads: u32,
    total_bases: u64,
    target_region_offered: bool,
    bases_on_target: u64,
    bases_near_target: u64,
    total_effective_sequences: u64,
    effective_sequences_on_target: u64,
    effective_sequences_near_target: u64,
    average_depth: u16,
    average_depth_on_target: u16,
    average_depth_near_target: u16,
    mismatch_bases: u64,
    mismatch_bases_on_target: u64,
    bases_of_target: u64,
    bases_of_target_covered: u64,
    bases_near_target_covered: u64,
    flank_size: u16,
    bases_of_flank: u64,
    reads_covers_flank: u32,
    bases_mapped_on_flank: u64,
}

/*
enum SamFlagSpecs {
    multiple_segments = 1u16,
    each_properly_aliged = 2u16,
    sgement_unmapped = 4u16,
    next_segment_unmapped = 8u16,
    segment_reverse_strand = 16u16,
    next_segment_reverse_strand = 32u16,
    first_segment = 64u16,
    last_segment = 128u16,
    secondary_aligment = 256u16,
    failed_filter_segment = 512u16,
    pcr_or_optical_dups = 1024u16,
    supplymentary_segment = 2048u16,
}

struct SamFlags {
    multiple_segments: bool,
    each_properly_aliged: bool,
    sgement_unmapped: bool,
    next_segment_unmapped: bool,
    segment_reverse_strand: bool,
    next_segment_reverse_strand: bool,
    first_segment: bool,
    last_segment: bool,
    secondary_aligment: bool,
    failed_filter_segment: bool,
    pcr_or_optical_dups: bool,
    supplymentary_segment: bool,
}

impl SamFlags {
    fn is_d
}
*/

// 1. file -> mapping quality rate stats
// 2. file -> mapping coverage rate stats
// 3. file -> mapping depth rate stats
// 4. file -> chromosome depth and coverage
// 5. file -> For each region in probe file (in.bed)

// fn summary_on_region() -> Target {}

fn main() {
    let pargs: clap::ArgMatches = App::new("bsum")
        .version("0.0.1")
        .author("liuxiaochuan@novogene.com")
        .about("Get summary info on bam with/without target region")
        .setting(AppSettings::ArgRequiredElseHelp)
        .args_from_usage(
            "
            [region] -r, --region=[file] 'a optional region bed'
            [flank] -f, --flank=[number] 'default as 200, unit bp'
            <input> 'input bam file'
            ",
        )
        .get_matches();
    let mut target: Target = Default::default();
    let mut reader = match BamReader::from_path(pargs.value_of("input").unwrap()) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", serde_json::to_string_pretty(&target).unwrap());
            panic!("error reading input bam: {}", e)
        }
    };
    let bed: Option<&Path> = match pargs.value_of("input") {
        Some(v) => {
            let v = &Path::new(v);
            if v.exists() {
                target.target_region_offered = true;
                Some(v)
            } else {
                println!("{}", serde_json::to_string_pretty(&target).unwrap());
                panic!("path {:?} does not exists", v.to_str())
            }
        }
        None => {
            target.target_region_offered = false;
            None
        }
    };
    let flank_len: u16 = pargs.value_of("flank").unwrap_or("200").parse().unwrap();
    for i in reader.records() {
        let rec: Record = match i {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        };
        target.total_reads += 1;
        if rec.is_paired() {
            target.paired_reads += 1;
        };
        if rec.is_quality_check_failed() {
            target.failed_qc_reads += 1;
        };
        if rec.is_duplicate() {
            target.duplicate_reads += 1;
        };
        // related to mapping
        if !rec.is_unmapped() {
            target.mapped_reads += 1;
        };
        /*
        if rec.is_mapped() && rec.tid() < 25 {
            target.mapped_reads_25 += 1;
        };
        */
        if !rec.is_unmapped() && !rec.is_mate_unmapped() {
            target.pe_mapped_reads += 1;
            if rec.tid() != rec.mtid() {
                target.mate_on_diff_chr += 1;
            };
            if rec.tid() != rec.mtid() && rec.mapq() >= 5 {
                target.mate_on_diff_chr_q5 += 1;
            };
        } else if !rec.is_unmapped() {
            target.se_mapped_reads += 1;
        };
        if rec.is_proper_pair() {
            target.properly_mapped_reads += 1;
        };
        if rec.is_first_in_template() {
            target.first_reads += 1;
        };
        if rec.is_last_in_template() {
            target.second_reads += 1;
        };
        if rec.is_reverse() {
            target.forward_strand_reads += 1;
        } else {
            target.backward_strand_reads += 1;
        };
        if rec.is_secondary() {
            target.secondary_reads += 1;
        };
        if rec.is_supplementary() {
            target.supplymentary_reads += 1;
        };
        // base
        target.total_bases += rec.seq().len() as u64;
    }
    println!("{}", serde_json::to_string_pretty(&target).unwrap());
}
