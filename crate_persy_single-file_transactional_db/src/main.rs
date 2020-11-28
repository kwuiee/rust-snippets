use std::error::Error;

use persy::{Config, Persy};

fn main() -> Result<(), Box<dyn Error>> {
    Persy::create("/tmp/open.persy")?;
    let persy = Persy::open("/tmp/open.persy", Config::new())?;
    let mut tx = persy.begin()?;
    tx.create_segment("seg")?;
    let data = vec![1; 20];
    tx.insert("seg", &data)?;
    let prepared = tx.prepare()?;
    prepared.commit()?;
    for (_id, content) in persy.scan("seg")? {
        assert_eq!(content[0], 1);
    }
    Ok(())
}
