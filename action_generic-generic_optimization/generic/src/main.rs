// Develeper A
use std::fmt::Debug;

trait PersonT {
    type I: Debug;

    fn name(&self) -> String;
    fn age(&self) -> u32;
    fn interest(&self) -> Self::I;
}

fn self_intro<P: PersonT>(person: &P) {
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

impl PersonT for Person {
    type I = Interest;

    fn name(&self) -> String {
        self.name.clone()
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn interest(&self) -> Self::I {
        self.interest.clone()
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
