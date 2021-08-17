extern crate syn;

use syn::Type;

fn main() {
    let t: Type = syn::parse_str("std::collections::HashMap<String, Value>").unwrap();
    println!("{:#?}", t);
    let t: Type = syn::parse_str("println!(\"abc\")").unwrap();
    println!("{:#?}", t);
}
