extern crate bson;
extern crate mongodb;

use mongodb::options::{auth::AuthMechanism, auth::Credential, ClientOptions};
use mongodb::Client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut options = ClientOptions::parse("mongodb://39.97.111.44:8024")?;
    options.credential = Credential {
        username: Some("cancer".to_string()),
        source: Some("hwtj".to_string()),
        password: Some("yRQr!9#XY$".to_string()),
        mechanism: Some(AuthMechanism::ScramSha256),
    };
    let client = Client::with_options(options)?;
    let db = client.database("hwtj");
    for col_name in db.list_collection_names()? {
        println!("collection: {}", col_name);
    }
    Ok(())
}
