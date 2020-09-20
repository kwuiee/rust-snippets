use std::error::Error;

#[derive(serde_query::Deserialize)]
struct Data {
    #[query(".commit.authors.[0]")]
    first_author: String,
    #[query(".hash")]
    hash_value: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let document = serde_json::to_string(&serde_json::json!({
        "commit": {
            "authors": ["Kou", "Kasumi", "Masaru"],
            "date": "2020-09-10",
        },
        "hash": 0xabcd,
    }))?;

    // The query is compatible with arbitrary data formats with serde support.
    let data: Data = serde_json::from_str(&document)?;

    assert_eq!(data.first_author, "Kou");
    assert_eq!(data.hash_value, 0xabcd);
    Ok(())
}
