use bio::alignment::pairwise::*;
use bio::alignment::AlignmentOperation::*;
use bio::scores::blosum62;

fn main() {
    let x = b"ACCGTGGAT";
    let y = b"AAAAACCGTTGAT";
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    // gap open score: -5, gap extension score: -1
    let mut aligner = Aligner::with_capacity(x.len(), y.len(), -5, -1, &score);
    let alignment = aligner.semiglobal(x, y);
    println!("Semiglobal:\n{}", alignment.pretty(x, y));
    // x is global (target sequence) and y is local (reference sequence)
    assert_eq!(alignment.ystart, 4);
    assert_eq!(alignment.xstart, 0);
    assert_eq!(
        alignment.operations,
        [Match, Match, Match, Match, Match, Subst, Match, Match, Match]
    );

    // You can use predefined scoring matrices such as BLOSUM62
    let x = b"LSPADKTNVKAA";
    let y = b"PEEKSAV";
    // gap open score: -10, gap extension score: -1
    let mut aligner = Aligner::with_capacity(x.len(), y.len(), -10, -1, &blosum62);
    let alignment = aligner.local(x, y);
    println!("Local:\n{}", alignment.pretty(x, y));
    assert_eq!(alignment.xstart, 2);
    assert_eq!(alignment.xend, 9);
    assert_eq!(alignment.ystart, 0);
    assert_eq!(alignment.yend, 7);
    assert_eq!(
        alignment.operations,
        [Match, Subst, Subst, Match, Subst, Subst, Match]
    );
    assert_eq!(alignment.score, 16);

    // If you don't know sizes of future sequences, you could
    // use Aligner::new().
    // Global alignment:
    let mut aligner = Aligner::new(-5, -1, &score);
    let x = b"ACCGTGGAT";
    let y = b"AAAAACCGTTGAT";
    let alignment = aligner.global(x, y);
    println!("Global:\n{}", alignment.pretty(x, y));
    assert_eq!(alignment.ystart, 0);
    assert_eq!(alignment.xstart, 0);
    assert_eq!(aligner.local(x, y).score, 7);

    // In addition to the standard modes (Global, Semiglobal and Local), a custom alignment
    // mode is supported which supports a user-specified clipping penalty. Clipping is a
    // special boundary condition where you are allowed to clip off the beginning/end of
    // the sequence for a fixed penalty. As a starting example, we can use the custom mode
    // for achieving the three standard modes as follows.

    // scoring for semiglobal mode
    let scoring = Scoring::new(-5, -1, &score) // Gap open, gap extend and match score function
        .xclip(MIN_SCORE) // Clipping penalty for x set to 'negative infinity', hence global in x
        .yclip(0); // Clipping penalty for y set to 0, hence local in y
    let mut aligner = Aligner::with_scoring(scoring);
    let alignment = aligner.custom(x, y); // The custom aligner invocation
    println!("Custom:\n{}", alignment.pretty(x, y));
    assert_eq!(alignment.ystart, 4);
    assert_eq!(alignment.xstart, 0);
    // Note that in the custom mode, the clips are explicitly mentioned in the operations
    assert_eq!(
        alignment.operations,
        [
            Yclip(4),
            Match,
            Match,
            Match,
            Match,
            Match,
            Subst,
            Match,
            Match,
            Match
        ]
    );

    // scoring for global mode
    // scoring can also be created using from_scores if the match and mismatch scores are constants
    let scoring = Scoring::from_scores(-5, -1, 1, -1) // Gap open, extend, match, mismatch score
        .xclip(MIN_SCORE) // Clipping penalty for x set to 'negative infinity', hence global in x
        .yclip(MIN_SCORE); // Clipping penalty for y set to 'negative infinity', hence global in y
    let mut aligner = Aligner::with_scoring(scoring);
    let alignment = aligner.custom(x, y); // The custom aligner invocation
    println!("Custom:\n{}", alignment.pretty(x, y));
    assert_eq!(alignment.ystart, 0);
    assert_eq!(alignment.xstart, 0);
    // Note that in the custom mode, the clips are explicitly mentioned in the operations
    assert_eq!(
        alignment.operations,
        [Del, Del, Del, Del, Match, Match, Match, Match, Match, Subst, Match, Match, Match]
    );

    // Similarly if the clip penalties are both set to 0, we have local alignment mode. The scoring
    // struct also lets users set different penalties for prefix/suffix clipping, thereby letting
    // users have the flexibility to create a wide variety of boundary conditions. The xclip() and
    // yclip() methods sets the prefix and suffix penalties to be equal. The scoring struct can be
    // explicitly constructed for full flexibility.

    // The following example considers a modification of the semiglobal mode where you are allowed
    // to skip a prefix of the target sequence x, for a penalty of -10, but you have to consume
    // the rest of the string in the alignment

    let scoring = Scoring {
        gap_open: -5,
        gap_extend: -1,
        match_fn: |a: u8, b: u8| if a == b { 1i32 } else { -3i32 },
        match_scores: Some((1, -3)),
        xclip_prefix: -10,
        xclip_suffix: MIN_SCORE,
        yclip_prefix: 0,
        yclip_suffix: 0,
    };
    let x = b"GGGGGGACGTACGTACGT";
    let y = b"AAAAACGTACGTACGTAAAA";
    let mut aligner = Aligner::with_capacity_and_scoring(x.len(), y.len(), scoring);
    let alignment = aligner.custom(x, y);
    println!("Custom:\n{}", alignment.pretty(x, y));
    assert_eq!(alignment.score, 2);
    assert_eq!(
        alignment.operations,
        [
            Yclip(4),
            Xclip(6),
            Match,
            Match,
            Match,
            Match,
            Match,
            Match,
            Match,
            Match,
            Match,
            Match,
            Match,
            Match,
            Yclip(4)
        ]
    );
}
