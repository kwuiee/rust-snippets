extern crate rust_htslib;

use std::error::Error;

use rust_htslib::bcf::header::Header;
use rust_htslib::bcf::record::GenotypeAllele;
use rust_htslib::bcf::{Format, Writer};

fn main() -> Result<(), Box<dyn Error>> {
    // Create minimal VCF header with a single contig and a single sample
    let mut header = Header::new();
    let header_contig_line = r#"##contig=<ID=1,length=10>"#;
    header.push_record(header_contig_line.as_bytes());
    let header_gt_line = r#"##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">"#;
    header.push_record(header_gt_line.as_bytes());
    header.push_sample("test_sample".as_bytes());

    // Write uncompressed VCF to stdout with above header and get an empty record
    let mut vcf = Writer::from_stdout(&header, true, Format::VCF).unwrap();
    let mut record = vcf.empty_record();

    // Set chrom and pos to 1 and 7, respectively - note the 0-based positions
    let rid = vcf.header().name2rid(b"1").unwrap();
    record.set_rid(Some(rid));
    record.set_pos(6);

    // Set record genotype to 0|1 - note first allele is always unphased
    let alleles = &[GenotypeAllele::Unphased(0), GenotypeAllele::Phased(1)];
    record.push_genotypes(alleles).unwrap();

    // Write record
    vcf.write(&record).unwrap();

    Ok(())
}
