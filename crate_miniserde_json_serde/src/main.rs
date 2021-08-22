use miniserde::{json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    code: u32,
    message: String,
}

fn main() -> miniserde::Result<()> {
    let example = Example {
        code: 200,
        message: "reminiscent of Serde".to_owned(),
    };

    let j = json::to_string(&example);
    println!("{}", j);

    let out: Example = json::from_str(&j)?;
    println!("{:?}", out);

    Ok(())
}
