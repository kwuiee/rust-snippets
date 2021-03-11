use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn main() -> Result<(), serde_yaml::Error> {
    let point = Point { x: 1.0, y: 2.0 };

    let s = serde_yaml::to_string(&point)?;
    assert_eq!(s, "---\nx: 1.0\ny: 2.0\n");

    let deserialized_point: Point = serde_yaml::from_str(&s)?;
    assert_eq!(point, deserialized_point);
    Ok(())
}
