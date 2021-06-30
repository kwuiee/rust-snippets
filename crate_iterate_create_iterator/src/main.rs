use iterate::iterate;

fn main() {
    let range = 0..5;
    let vec = vec![4, 1, 2, 3];

    // Iterate captures its arguments by move, and evaluates them lazily, so we
    // need to ensure that the vec is captured by reference
    let vec = &vec;

    let iterator = iterate![1, ..range, ..vec.iter().copied(), 10];

    let result: Vec<i32> = iterator.collect();
    assert_eq!(result, [1, 0, 1, 2, 3, 4, 4, 1, 2, 3, 10,])
}
