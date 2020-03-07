// Develeper A
use std::fmt::Debug;

trait PersonT<'a> {
    type I: Debug;

    fn name(&'a self) -> &'a str;
    fn age(&self) -> u32;
    fn interest(&'a self) -> &'a Self::I;
}

fn self_intro<'a, P: PersonT<'a>>(person: &'a P) {
    println!(
        "My name is {}, I'm {} years old, I like {:?}.",
        person.name(),
        person.age(),
        person.interest()
    );
}

#[derive(Debug, Clone)]
enum Interest {
    Basketball,
    Football,
}

struct Person {
    name: String,
    age: u32,
    interest: Interest,
}

impl<'a> PersonT<'a> for Person {
    type I = Interest;

    fn name(&'a self) -> &'a str {
        &self.name
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn interest(&'a self) -> &'a Self::I {
        &self.interest
    }
}

// Developer B
fn main() {
    let p = Person {
        name: "Zhangsan".to_string(),
        age: 18,
        interest: Interest::Basketball,
    };
    self_intro(&p);
}
