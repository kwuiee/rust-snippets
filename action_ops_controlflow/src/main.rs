#![feature(control_flow_enum)]

use std::ops::ControlFlow;

fn main() {
    let mut partial_sum = 0;
    (1..10).try_for_each(|x| {
        println!("{}", x);
        if partial_sum > 30 {
            ControlFlow::BREAK
        } else {
            partial_sum += x;
            ControlFlow::CONTINUE
        }
    });
    assert_eq!(partial_sum, 36);
}
