use skytable::actions::Actions;
use skytable::query;
use skytable::sync::Connection;
use skytable::types::RawString;

fn main() {
    let mut con = Connection::new("127.0.0.1", 2003).unwrap();
    con.set("hello", "world").unwrap();
    let x: String = con.get("hello").unwrap();
    assert_eq!(x, "world");
    let mybin = RawString::from(vec![1, 2, 3, 4]);
    con.set("bindata", mybin).unwrap();
    let x: String = con.get("bindata").unwrap();
    println!("{:?}", x);
}
