fn main() {
    let text = "textwrap: an efficient and powerful library for wrapping text.";
    println!("{}", textwrap::fill(text, 28));
}
