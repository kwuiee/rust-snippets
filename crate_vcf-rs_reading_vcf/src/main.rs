extern crate vcf;

use std::fs::File;
use vcf::*;

fn main() -> Result<(), VCFParseError> {
    let mut vcf_reader = VCFReader::new(
        File::open("data/Sample_A_dellySV.vcf").map_err(|e| VCFParseError::IoError { error: e })?,
    )?;
    assert_eq!(vcf_reader.header().items.len(), 255);
    assert_eq!(
        vcf_reader.header().items[0],
        VCFHeaderLine {
            line: "##fileformat=VCFv4.1".to_string(),
            contents: VCFHeaderContent::FileFormat("VCFv4.1".to_string())
        }
    );
    assert_eq!(
        vcf_reader.header().samples,
        vec!["HG00096", "HG00097", "HG00099"]
    );

    // load next record
    let record = vcf_reader.iter().next().unwrap()?;
    assert_eq!(record.chromosome, "20");
    assert_eq!(record.position, 34001111);
    assert_eq!(record.id, vec!["rs565014200"]);
    assert_eq!(record.reference, "T");
    assert_eq!(record.alternative, vec!["C"]);
    assert_eq!(record.quality, Some("100".to_string()));
    assert_eq!(record.info["AN"], vec!["6"]); // vcf-rs does not validate a number of entries and type
    assert_eq!(record.call["HG00096"]["GT"], vec!["0|0"]);
    Ok(())
}
