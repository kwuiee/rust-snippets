extern crate uom;

use uom::si::f32::*;
use uom::si::length::kilometer;
use uom::si::time::second;

fn main() {
    let length = Length::new::<kilometer>(5.0);
    let time = Time::new::<second>(15.0);
    let velocity/*: Velocity*/ = length / time;
    let acceleration = calc_acceleration(velocity, time);
    //let error = length + time; // error[E0308]: mismatched types
}

fn calc_acceleration(velocity: Velocity, time: Time) -> Acceleration {
    velocity / time
}
