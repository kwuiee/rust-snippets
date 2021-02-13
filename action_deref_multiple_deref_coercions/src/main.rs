use std::ops::Deref;

struct A(i32);
struct B(A);
struct C(B);

impl Deref for A {
    type Target = i32;

    fn deref(&self) -> &i32 {
        println!("deref A to i32");
        &self.0
    }
}

impl Deref for B {
    type Target = A;

    fn deref(&self) -> &A {
        println!("deref B to A");
        &self.0
    }
}

impl Deref for C {
    type Target = B;

    fn deref(&self) -> &B {
        println!("deref C to B");
        &self.0
    }
}

fn print_number(number: &i32) {
    println!("number = {}", number);
}

fn main() {
    let c: C = C(B(A(13)));
    print_number(&c);
}
