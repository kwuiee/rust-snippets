use infinitree::{
    anyhow,
    backends::Directory,
    fields::{LocalField, Serialized, VersionedMap},
    Index, Infinitree, Key,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlantHealth {
    id: usize,
    air_humidity: usize,
    soil_humidity: usize,
    temperature: f32,
}

#[derive(Index, Default, Clone)]
pub struct Measurements {
    // rename the field when serializing
    #[infinitree(name = "last_time")]
    _old_last_time: Serialized<String>,

    #[infinitree(name = "last_time2")]
    last_time: Serialized<usize>,

    // only store the keys in the index, not the values
    #[infinitree(strategy = "infinitree::fields::SparseField")]
    measurements: VersionedMap<usize, PlantHealth>,

    // skip the next field when loading & serializing
    #[infinitree(skip)]
    current_time: usize,
}

fn main() -> anyhow::Result<()> {
    let mut tree = Infinitree::<Measurements>::empty(
        Directory::new("./storage")?,
        Key::from_credentials("username", "password")?,
    );

    tree.index().measurements.insert(
        1,
        PlantHealth {
            id: 0,
            air_humidity: 50,
            soil_humidity: 60,
            temperature: 23.3,
        },
    );

    *tree.index().last_time.write() = 1;
    tree.commit("first measurement! yay!")?;
    Ok(())
}
