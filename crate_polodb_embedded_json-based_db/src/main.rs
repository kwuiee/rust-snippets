#[macro_use]
extern crate polodb_bson;

use std::borrow::Borrow;
use std::rc::Rc;

use polodb_bson::Value;
use polodb_core::db::Database;
use polodb_core::DbErr;

fn main() -> Result<(), DbErr> {
    let mut db = Database::open("/tmp/polo.bson")?;
    let mut collection = db.create_collection("test")?;
    let count = collection.count()?;
    let new_doc = mk_document! {
        "name": "Vincent Chan",
    };
    collection.insert(Rc::new(new_doc))?;
    let result = collection.find(Some(
        mk_document! {
            "content": "3",
        }
        .borrow(),
    ))?;
    Ok(())
}
