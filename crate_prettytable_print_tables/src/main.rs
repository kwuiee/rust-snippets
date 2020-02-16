#[macro_use]
extern crate prettytable;

use prettytable::{Cell, Row, Table};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();

    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);

    table.add_row(Row::new(vec![
        Cell::new("foobar2"),
        Cell::new("bar2"),
        Cell::new("foo2"),
    ]));

    let out = File::create("output.csv")?;
    // 写入
    table.to_csv(out).unwrap();

    // 读取
    let table = Table::from_csv_file("output.csv")?;
    table.printstd();
    Ok(())
}
