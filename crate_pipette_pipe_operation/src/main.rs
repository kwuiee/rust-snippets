use pipette::{pipe, Pipeline};

fn main() {
    let input = 1;

    let factor = (
        input,
        |a| a * 2,
        |a| a * 3,
        |a| a * 4,
        |a| a * 5,
        |a| a * 6,
        |a| a * 7,
        |a| a * 8,
    );
    let output = pipe(factor);

    assert_eq!(output, factor.compute());
    assert_eq!(output, 40_320);
}
