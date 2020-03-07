use std::borrow::Cow;

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        input
            .chars()
            .filter(|&x| x != ' ')
            .collect::<std::string::String>()
            .into()
    } else {
        input.into()
    }
}

fn main() {
    let s = remove_spaces("Herman Radtke");
    println!("Length of string is {}", s.len());
}
