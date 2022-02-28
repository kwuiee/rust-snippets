#[macro_use(concat_string)]
extern crate concat_string;

fn main() {
    println!("{}", concat_string!("Hello", String::from(" "), "world"));
}
