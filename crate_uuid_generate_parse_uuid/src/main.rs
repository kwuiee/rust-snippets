extern crate uuid;

use std::error::Error;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn Error>> {
    let my_uuid = Uuid::new_v4();
    println!("uuid {}", my_uuid.to_string());

    let uuid2 = Uuid::parse_str(&my_uuid.to_string())?;
    println!("{}", uuid2);
    Ok(())
}
