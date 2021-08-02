extern crate prettier_print;

use std::collections::HashMap;

use prettier_print::prettier_printer::PrettierPrinter;

fn main() {
    #[derive(Debug)]
    struct Type {
        a: String,
        b: Vec<i32>,
        c: HashMap<&'static str, &'static str>,
    }

    let input = Type {
        a: "a".to_string(),
        b: vec![0, 1],
        c: {
            let mut map = HashMap::new();
            map.insert("So", "pretty");
            map
        },
    };
    println!("{:?}", &input);
    println!("{}", PrettierPrinter::default().print(&input));
}
