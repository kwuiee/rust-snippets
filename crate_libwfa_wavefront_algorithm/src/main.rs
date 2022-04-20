// Original sequences
// GGGGGCTGCAGTCCTGTGGTGCCTCCGGGCAGATAGCTGAGCATTTCATCATTGTAACTGGACTCCAGGCTGATGATCTCATCAATGACATCATCAATCATGTCACTTCCCATCATGGAACCACTCATGGTTGCTGGTGGAACGCCAGGA
// CTCCGGGCAGATAGCTGAGCATTTCATCATTGTAACTGGACTCCAGGCTGATGATCTCATCAATGACATCATCAATCATGTCACTTCCCATCATGGAACCACTCATGGTTGCTGGTGGAACGCCAGGATTAGCTTCATAACCTATGCCAC
// GGGTGATTATAGAGGCTTTGGCATAGGATCAGAAATTCTGAAGAATCTAAGCCAGGTTGCAATGAGGTGTCGCTGCAGCAGCATGCACTTCTTGGTAGCAGAATGGAATGTACCAGGACACCATGGGACCCTCCCAGCATGTCTATGCCT
use libwfa::{affine_wavefront::*, bindings::*, mm_allocator::*, penalties::*};

fn main() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let querys = vec![
        String::from("GGGGGCTGCAGTCCTGTGGTGCCTCCGGGCAGATAGCTGAGCATTTCATCATTGTAACTGGACTCCAGGCTGATGATCTCATCAATGACATCATCAATCATGTCACTTCCCATCATGGAACCACTCATGGTTGCTGGTGGAACGCCAGGA").into_bytes(), 
        String::from("CTCCGGGCAGATAGCTGAGCATTTCATCATTGTAACTGGACTCCAGGCTGATGATCTCATCAATGACATCATCAATCATGTCACTTCCCATCATGGAACCACTCATGGTTGCTGGTGGAACGCCAGGATTAGCTTCATAACCTATGCCAC").into_bytes(),
        String::from("GGGTGATTATAGAGGCTTTGGCATAGGATCAGAAATTCTGAAGAATCTAAGCCAGGTTGCAATGAGGTGTCGCTGCAGCAGCATGCACTTCTTGGTAGCAGAATGGAATGTACCAGGACACCATGGGACCCTCCCAGCATGTCTATGCCT").into_bytes()
    ];
    let pattern = concat!(include_str!("TFE3"), include_str!("SFPQ"))
        .replace('\n', "")
        .into_bytes();

    for query in querys {
        let mut penalties = AffinePenalties {
            match_: 0,
            mismatch: 2,
            gap_opening: 10,
            gap_extension: 2,
        };

        let pat_len = pattern.len();
        let text_len = query.len();

        let mut wavefronts =
            AffineWavefronts::new_complete(pat_len, text_len, &mut penalties, &alloc);
        // let mut wavefronts = AffineWavefronts::new_reduced(pat_len, text_len, &mut penalties, 20, 10, &alloc);

        wavefronts.align(&pattern, &query).unwrap();

        let score = wavefronts.edit_cigar_score(&mut penalties);
        println!("score: {}", score);

        // wavefronts.print_cigar(pattern, query);

        // Prettier byte vector
        let cigar = wavefronts.cigar_bytes();
        println!("cigar: {}", std::str::from_utf8(&cigar).unwrap());

        // The cigar can also be extracted as a byte vector
        let cigar = wavefronts.cigar_bytes_raw();
        // println!("{}", std::str::from_utf8(&cigar).unwrap());

        let mut pattern_padded: Vec<u8> = vec![];
        let mut query_padded: Vec<u8> = vec![];
        let mut query_iter = query.iter();
        let mut pattern_iter = pattern.iter();
        cigar.iter().for_each(|byte| match byte {
            b'D' => {
                query_padded.push(b'-');
                pattern_padded.push(*pattern_iter.next().unwrap())
            }
            b'I' => {
                pattern_padded.push(b'-');
                query_padded.push(*query_iter.next().unwrap())
            }
            _ => {
                query_padded.push(*query_iter.next().unwrap());
                pattern_padded.push(*pattern_iter.next().unwrap())
            }
        });

        unsafe {
            println!(
                "{}\n{}\n{}",
                std::str::from_utf8_unchecked(&pattern_padded),
                std::str::from_utf8_unchecked(&cigar),
                std::str::from_utf8_unchecked(&query_padded)
            );
        }
    }
}
