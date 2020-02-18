extern crate bson;

use bson::{decode_document, encode_document, Bson, Document};
use std::error::Error;
use std::fs::File;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn Error>> {
    let mut doc = Document::new();
    doc.insert("foo".to_owned(), Bson::String("bar".to_owned()));

    let mut buf = Vec::new();
    let mut reader = File::create("target/test.bson")?;
    encode_document(&mut buf, &doc)?;
    encode_document(&mut reader, &doc)?;

    let doc = decode_document(&mut Cursor::new(&buf[..]))?;
    println!("{}", doc);
    Ok(())
}
