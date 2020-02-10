extern crate reqwest;

fn main() {
    let body = reqwest::get("https://www.rust-lang.org")
        .unwrap()
        .text()
        .unwrap();

    println!("body = {:?}", body);
}
