//!
//! [reference](https://doc.rust-lang.org/nightly/std/primitive.char.html)
//!
fn main() {
    // alpha
    let s = "\u{03B1}";
    println!("{}", s);
    // thumb up
    let s = "\u{01F44D}";
    println!("{}", s);
    // face palm
    println!("utf16 len: {}", "\u{01F926}".len());
    // cross: x
    println!("{}", "\u{2717}");
    println!("{}", "\u{2612}");
    // tick: v
    println!("{}", "\u{2713}");
    println!("{}", "\u{2714}");
    println!("{}", "\u{2611}");
}
