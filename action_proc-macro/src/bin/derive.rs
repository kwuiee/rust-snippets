use derive_macro::DeriveHello;

trait HelloTrait {
    fn say_hello();
}

#[derive(DeriveHello)]
struct Pancakes;

fn main() {
    Pancakes::say_hello();
}
