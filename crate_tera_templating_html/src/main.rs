extern crate serde;
extern crate tera;

use serde::Serialize;
use std::error::Error;
use tera::{Context, Tera};

#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
    from: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let u = User {
        name: "Sean Lyo".to_owned(),
        age: 30u8,
        from: "HD".to_owned(),
    };
    let mut context: Context = Context::new();
    context.insert("name", "nasdaq wellcome genetronhealth");
    context.insert("user", &u);
    context.insert("numbers", &[1, 2, 3, 4, 5]);
    context.insert("num", &8);
    let tera = Tera::new("templates/*.html")?;
    println!("{}", tera.render("base.html", &context)?);
    Ok(())
}
