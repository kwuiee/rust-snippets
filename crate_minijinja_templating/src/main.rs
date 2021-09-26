extern crate minijinja;

use std::collections::HashMap;

use minijinja::Environment;
use serde::Serialize;

#[derive(Serialize)]
pub struct Context {
    name: String,
}

fn main() {
    let mut env = Environment::new();
    env.add_template("hello.txt", "Hello {{ name }}!").unwrap();
    let template = env.get_template("hello.txt").unwrap();
    println!(
        "{}",
        template
            .render(&Context {
                name: "World".into()
            })
            .unwrap()
    );
    let mut map = HashMap::new();
    map.insert("name", "Joe");
    println!("{}", template.render(&map).unwrap());
}
