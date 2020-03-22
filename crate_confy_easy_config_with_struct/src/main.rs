#[macro_use]
extern crate serde;
extern crate confy;

#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    version: u8,
    api_key: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            version: 6,
            api_key: "DF2348AE".into(),
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: MyConfig = confy::load("my-app-name")?;
    dbg!(cfg);
    Ok(())
}
