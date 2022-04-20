// rust_wfa
use lib::{
    alignment_lib::AlignResult, alignment_lib::Penalties, wavefront_alignment::wavefront_align,
};

fn main() {
    let query = "CTCCGGGCAGATAGCTGAGCATTTCATCATTGTAACT-GGACTCCAGGCTGATGATCTCATCAATGACATCATCAATCATGTCACTTCCCATCATGGAACCACTCATGGTTGCTGGTGGAACGCCAGGATTAGCTTCATAACCTATGCCAC";
    let text = concat!(include_str!("TFE3"), include_str!("SFPQ"));
    let pens = Penalties {
        mismatch_pen: 1,
        open_pen: 2,
        extd_pen: 2,
    };

    match wavefront_align(query, text, &pens) {
        AlignResult::Res(alignment) => {
            println!(
                "Alignment score {}\n{}\n{}",
                alignment.score, alignment.query_aligned, alignment.text_aligned
            );
        }
        AlignResult::Error(alignerr) => {
            println!("{:?}", alignerr)
        }
    };
}
