#[macro_use]
extern crate derive_attr;

trait THelloWord {
    fn hello();
}

#[derive(HelloWorld)]
struct FrenchToast;

fn main() {
    FrenchToast::hello();
}
