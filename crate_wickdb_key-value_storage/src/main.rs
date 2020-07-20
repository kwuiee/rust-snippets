extern crate wickdb;

use wickdb::file::FileStorage;
use wickdb::Options;
use wickdb::ReadOptions;
use wickdb::WickDB;
use wickdb::WriteOptions;
use wickdb::DB;

fn main() {
    let options = Options::default();
    let storage = FileStorage::default();
    let db = WickDB::open_db(options, "/tmp/test_db", storage).expect("could not open db");
    db.put(
        WriteOptions::default(),
        "key1".as_bytes(),
        "value1".as_bytes(),
    )
    .expect("could not success putting");
    db.put(
        WriteOptions::default(),
        "key2".as_bytes(),
        "value2".as_bytes(),
    )
    .expect("could not success putting");
    let val1 = db
        .get(ReadOptions::default(), "key1".as_bytes())
        .expect("could not get key1");
    let val2 = db
        .get(ReadOptions::default(), "key2".as_bytes())
        .expect("could not get key2");
    assert!(val1.is_some());
    assert!(val2.is_some());
    assert_eq!(val1.unwrap().as_str(), "value1");
    assert_eq!(val2.unwrap().as_str(), "value2");
}
