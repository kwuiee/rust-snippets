use std::borrow::Cow;

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buf.push(c)
            }
        }
        Cow::Owned(buf)
    } else {
        Cow::Borrowed(input)
    }
}

fn main() {
    let s1 = "no_spaces_in_string";
    let mut result1 = remove_spaces(s1);
    println!("{}", result1);
    println!("{:p} {:p}", s1, result1.to_mut());
    let s2 = "spaces in string";
    let mut result2 = remove_spaces(s2);
    println!("{}", result2);
    println!("{:p} {:p}", s2, result2.to_mut());
}
