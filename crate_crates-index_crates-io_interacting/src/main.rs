extern crate crates_index;

fn main() {
    let index = crates_index::Index::new_cargo_default();
    if !index.exists() {
        index.retrieve().expect("Could not fetch crates.io index");
    }
    let crate_ = index.crate_("regex");
    println!(
        "crate name: {}, first version: {}, latest version: {}.",
        crate_.name(),
        crate_.earliest_version(),
        crate_.latest_version()
    );
    // Iterating all crates
    /*
    for crate_ in index.crates() {
        let latest_version = crate_.latest_version();
        println!("crate name: {}", latest_version.name());
        println!("crate version: {}", latest_version.version());
    }
    */
}
