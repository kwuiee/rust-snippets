#![feature(specialization)]

use std::fmt::Display;

trait Example {
    fn call(&self);
}

impl<T> Example for T {
    default fn call(&self) {
        println!("most generic");
    }
}

impl Example for str {
    default fn call(&self) {
        println!("Specialized for str: {}", &self);
    }
}

impl<T> Example for T
where
    T: Display,
{
    default fn call(&self) {
        println!("Display: {}", &self);
    }
}

fn main() {
    let v1 = 1i32;
    let v2 = "love";

    v1.call();
    v2.call();
}
