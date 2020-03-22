fn foo(words: &[&str]) {
    match words {
        ["Hello", "World", "!", ..] => println!("Hello World!"),
        ["Foo", "Bar", ..] => println!("Baz"),
        rest => println!("{:?}", rest),
    }
}

fn bar(words: &[&str]) {
    match words {
        // Ignore everything but the last element, which must be "!".
        [.., "!"] => println!("!!!"),

        // `start` is a slice of everything except the last element, which must be "z".
        [start @ .., "z"] => println!("starts with: {:?}", start),

        // `end` is a slice of everything but the first element, which must be "a".
        ["a", end @ ..] => println!("ends with: {:?}", end),

        rest => println!("{:?}", rest),
    }
}
