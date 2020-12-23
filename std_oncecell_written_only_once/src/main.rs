#![feature(once_cell)]

use std::lazy::OnceCell;

fn main() {
    // Intialize a empty `OnceCell`
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    // Fill inner if empty and return inner.
    let value: &String = cell.get_or_init(|| "Hello, World!".to_string());
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());

    // .set returns Ok(()) if the cell was empty and Err(value) if it was full.
    assert_eq!(
        cell.set("New string.".to_string()),
        Err("New string.".to_string())
    );
}
