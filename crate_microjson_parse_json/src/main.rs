extern crate microjson;

use microjson::JSONParsingError;
use microjson::JSONValue;
use microjson::JSONValueType;

fn main() -> Result<(), JSONParsingError> {
    let integer = JSONValue::parse("42")?;

    let value: isize = integer.read_integer()?;

    let string = JSONValue::parse("\"hello there\"")?;

    let value: &str = string.read_string()?;

    let input = r#" [0, 1, 2, 3, 4, 5] "#;

    let array = JSONValue::parse(input)?;

    for (n, item) in array.iter_array()?.enumerate() {
        let value = item.read_integer()?;
        assert_eq!(value, n as isize);
    }

    let input = r#" { "arr": [3, "foo", 3.625, false] } "#;

    let object = JSONValue::parse(input)?;

    assert_eq!(
        object
            .get_key_value("arr")?
            .iter_array()?
            .nth(2)
            .unwrap()
            .read_float()?,
        3.625
    );

    let input = r#" 3.1415 "#;

    let object = JSONValue::parse(input)?;

    match object.value_type {
        JSONValueType::String => {}
        JSONValueType::Number => {}
        JSONValueType::Object => {}
        JSONValueType::Array => {}
        JSONValueType::Bool => {}
        JSONValueType::Null => {}
    }

    Ok(())
}
