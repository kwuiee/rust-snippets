#[derive(Debug)]
enum Interest {
    Basketball,
    Football,
}

struct Person {
    name: String,
    age: u32,
    interest: Interest,
}

fn self_intro(person: &Person) {
    println!(
        "My name is {}, I'm {} years old, I like {:?}.",
        person.name, person.age, person.interest
    );
}

fn main() {
    let p = Person {
        name: "Zhangsan".to_string(),
        age: 18,
        interest: Interest::Basketball,
    };

    self_intro(&p);
}
