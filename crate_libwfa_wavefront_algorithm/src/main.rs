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
    let text = concat!(include_str!("TFE3"), include_str!("SFPQ"))
        .replace('\n', "")
        .into_bytes();

    for query in querys {
        let mut penalties = AffinePenalties {
            match_: 0,
            mismatch: 2,
            gap_opening: 10,
            gap_extension: 2,
        };

        let mut wavefronts =
            AffineWavefronts::new_complete(query.len(), text.len(), &mut penalties, &alloc);
        // let mut wavefronts = AffineWavefronts::new_reduced(query.len(), text.len(), &mut penalties, 20, 10, &alloc);

        wavefronts.align(&query, &text).unwrap();

        let score = wavefronts.edit_cigar_score(&mut penalties);
        println!("score: {}", score);

        // wavefronts.print_cigar(&query, &text);

        // Prettier byte vector
        let cigar = wavefronts.cigar_bytes();
        println!("cigar: {}", std::str::from_utf8(&cigar).unwrap());

        // The cigar can also be extracted as a byte vector
        let cigar = wavefronts.cigar_bytes_raw();
        // println!("{}", std::str::from_utf8(&cigar).unwrap());

        let mut padded_query: Vec<u8> = vec![];
        let mut padded_text: Vec<u8> = vec![];
        let mut textiter = text.iter();
        let mut queryiter = query.iter();
        cigar.iter().for_each(|byte| match byte {
            b'D' => {
                padded_text.push(b'-');
                padded_query.push(*queryiter.next().unwrap())
            }
            b'I' => {
                padded_query.push(b'-');
                padded_text.push(*textiter.next().unwrap())
            }
            _ => {
                // X: substitution
                // M: match
                padded_text.push(*textiter.next().unwrap());
                padded_query.push(*queryiter.next().unwrap())
            }
        });

        unsafe {
            println!(
                "{}\n{}\n{}",
                std::str::from_utf8_unchecked(&padded_text),
                std::str::from_utf8_unchecked(&cigar),
                std::str::from_utf8_unchecked(&padded_query),
            );
        }
    }
}
