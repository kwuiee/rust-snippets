extern crate itertools;

use itertools::EitherOrBoth::{Both, Right};
use itertools::Itertools;
use itertools::Tuples;
use std::ops::Range;

fn main() {
    // interleave
    let it = (1..7).interleave(vec![-1, -2]);
    itertools::assert_equal(it, vec![1, -1, 2, -2, 3, 4, 5, 6]);

    // zip
    let it = (0..1).zip_longest(1..3);
    itertools::assert_equal(it, vec![Both(0, 1), Right(2)]);

    // group
    let data = vec![1, 3, -2, -2, 1, 0, 1, 2];
    // groups:     |---->|------>|--------->|
    let mut data_grouped = Vec::new();
    for (key, group) in &data.into_iter().group_by(|elt| *elt >= 0) {
        data_grouped.push((key, group.collect()));
    }
    assert_eq!(
        data_grouped,
        vec![
            (true, vec![1, 3]),
            (false, vec![-2, -2]),
            (true, vec![1, 0, 1, 2])
        ]
    );

    // chunk
    let data = vec![1, 1, 2, -2, 6, 0, 3, 1];
    //chunk size=3 |------->|-------->|--->|
    for chunk in &data.into_iter().chunks(3) {
        // Check that the sum of each chunk is 4.
        assert_eq!(4, chunk.sum());
    }

    // tuple
    let mut v = Vec::new();
    for (a, b) in (1..5).tuples() {
        v.push((a, b));
    }
    assert_eq!(v, vec![(1, 2), (3, 4)]);
    let mut it = (1..7).tuples();
    assert_eq!(Some((1, 2, 3)), it.next());
    assert_eq!(Some((4, 5, 6)), it.next());
    assert_eq!(None, it.next());
    // this requires a type hint
    let it = (1..7).tuples::<(_, _, _)>();
    itertools::assert_equal(it, vec![(1, 2, 3), (4, 5, 6)]);
    // you can also specify the complete type
    let it: Tuples<Range<u32>, (u32, u32, u32)> = (1..7).tuples();
    itertools::assert_equal(it, vec![(1, 2, 3), (4, 5, 6)]);

    // tee
    let xs = vec![0, 1, 2, 3];
    let (mut t1, t2) = xs.into_iter().tee();
    itertools::assert_equal(t1.next(), Some(0));
    itertools::assert_equal(t2, 0..4);
    itertools::assert_equal(t1, 1..4);

    // map
    (1i32..42i32).map_into::<f64>().collect_vec();
    let input = vec![Ok(41), Err(false), Ok(11)];
    let it = input.into_iter().map_ok(|i| i + 1);
    itertools::assert_equal(it, vec![Ok(42), Err(false), Ok(12)]);

    // filter
    let input = vec![Ok(22), Err(false), Ok(11)];
    let it = input.into_iter().filter_ok(|&i| i > 20);
    itertools::assert_equal(it, vec![Ok(22), Err(false)]);
    let input = vec![Ok(22), Err(false), Ok(11)];
    let it = input
        .into_iter()
        .filter_map_ok(|i| if i > 20 { Some(i * 2) } else { None });
    itertools::assert_equal(it, vec![Ok(44), Err(false)]);

    // cartesian product
    let it = (0..2).cartesian_product("αβ".chars());
    itertools::assert_equal(it, vec![(0, 'α'), (0, 'β'), (1, 'α'), (1, 'β')]);

    // coalesce
    let data = vec![-1., -2., -3., 3., 1., 0., -1.];
    itertools::assert_equal(
        data.into_iter().coalesce(|x, y| {
            if (x >= 0.) == (y >= 0.) {
                Ok(x + y)
            } else {
                Err((x, y))
            }
        }),
        vec![-6., 4., -1.],
    );

    // dedup
    let data = vec![
        (0, 1.),
        (1, 1.),
        (0, 2.),
        (0, 3.),
        (1, 3.),
        (1, 2.),
        (2, 2.),
    ];
    itertools::assert_equal(
        data.into_iter().dedup_by(|x, y| x.1 == y.1),
        vec![(0, 1.), (0, 2.), (0, 3.), (1, 2.)],
    );
    let data = vec!['a', 'a', 'b', 'c', 'c', 'b', 'b'];
    itertools::assert_equal(
        data.into_iter().dedup_with_count(),
        vec![(2, 'a'), (1, 'b'), (2, 'c'), (2, 'b')],
    );
    let mut hexadecimals = "0123456789abcdef".chars();

    // take while ref
    let decimals = hexadecimals
        .take_while_ref(|c| c.is_numeric())
        .collect::<String>();
    assert_eq!(decimals, "0123456789");
    assert_eq!(hexadecimals.next(), Some('a'));

    // combination & permutation
    let it = (1..5).combinations(3);
    itertools::assert_equal(
        it,
        vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]],
    );
    let perms = (5..8).permutations(2);
    itertools::assert_equal(
        perms,
        vec![
            vec![5, 6],
            vec![5, 7],
            vec![6, 5],
            vec![6, 7],
            vec![7, 5],
            vec![7, 6],
        ],
    );

    // k smallest
    let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
    let five_smallest = numbers.into_iter().k_smallest(5);
    itertools::assert_equal(five_smallest, 0..5);

    // and more...
}
