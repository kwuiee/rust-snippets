use bio::alignment::Alignment;
use bio::pattern_matching::myers::long;
use bio::pattern_matching::myers::Myers;
use bio::pattern_matching::myers::MyersBuilder;

fn main() {
    // Obtaining the starting position of a match
    let text = b"CGGTCCTGAGGGATTAGCAC";
    let pattern = b"TCCTAGGGC";
    let mut myers = Myers::<u64>::new(pattern);
    let occ: Vec<_> = myers.find_all(text, 2).collect();
    assert_eq!(occ, [(3, 12, 2), (3, 13, 2)]);

    // Obtaining alignments
    let text = b"CGGTCCTGAGGGATTAGCAC";
    let pattern = b"TCCTAGGGC";
    let mut myers = Myers::<u64>::new(pattern);
    // create an 'empty' alignment instance, which can be reused
    let mut aln = Alignment::default();
    let mut matches = myers.find_all(text, 3);
    while matches.next_alignment(&mut aln) {
        println!(
            "Hit fond in range: {}..{} (distance: {})",
            aln.ystart, aln.yend, aln.score
        );
        println!("{}", aln.pretty(pattern, text));
    }

    // Finding the best hit
    let text = b"CGGTCCTGAGGGATTAGCAC";
    let pattern = b"TCCTAGGGC";
    let mut myers = Myers::<u64>::new(pattern);
    let mut aln = Alignment::default();
    let mut matches = myers.find_all_lazy(text, 2);
    // first, find the best hit
    let (best_end, _) = matches.by_ref().min_by_key(|&(_, dist)| dist).unwrap();
    // now calculate the alignment
    matches.alignment_at(best_end, &mut aln);
    println!(
        "Best alignment at {}..{} (distance: {})",
        aln.ystart, aln.yend, aln.score
    );
    println!("{}", aln.pretty(pattern, text));

    // Dealing with ambiguities
    let text = b"GTCTGATCTTACC";
    let pattern = b"TGATCNT";
    let myers = MyersBuilder::new().ambig(b'N', b"ACGT").build_64(pattern);
    assert_eq!(myers.distance(text), 0);

    // Dealing with long sequence
    let text = b"GCACGAGGGTTTACCAGATTTTCTATGGGGATCATGTATTCATCAGCTATCTTGGTTTACCAGATTTTCTATGGGGATCATGTATTCATCAGCTATCTTGGTATTCAAGTCCATTCCTCTAACAATTCCTGGGCGTCCTTTCCCATAAAA";
    let pattern = b"AAGTAGATGTAATTGGTTTCTGCGTGAAAGTCTAAAGCACGAGGGTTTACCAGATTTTCTATGGGGATCATGTATTCATCAGCTATCTTGGTTTACCAGATTTTCTATGGGGATCATGTATTCATCAGCTATCTTGGTATTCAAGTCCAT";
    let mut myers = long::Myers::<u64>::new(pattern);
    let mut aln = Alignment::default();
    let mut matches = myers.find_all_lazy(text, 150);
    // first, find the best hit
    let (best_end, _) = matches.by_ref().min_by_key(|&(_, dist)| dist).unwrap();
    // now calculate the alignment
    matches.alignment_at(best_end, &mut aln);
    println!(
        "Best alignment at {}..{} (distance: {})",
        aln.ystart, aln.yend, aln.score
    );
    println!("{}", aln.pretty(pattern, text));
}
