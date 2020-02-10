#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::{Serialize, Serializer};

#[derive(Serialize)]
struct SerialTest {
    #[serde(serialize_with = "serialize_array64")]
    test: Vec<u8>,
}

fn serialize_array64<S>(array: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    array.serialize(serializer)
}

fn main() {
    let mut serial = SerialTest { test: vec![0; 64] };

    serial.test[35] = 12;

    let serialized = serde_json::to_string_pretty(&serial).unwrap();

    println!("serialized = {}", serialized);
}
