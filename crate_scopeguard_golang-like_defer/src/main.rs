#[macro_use(defer)]
extern crate scopeguard;

use std::cell::Cell;
use std::fs::*;
use std::io::{self, Write};

fn guard_file() -> io::Result<()> {
    let f = File::create("newfile.txt")?;
    let mut file = scopeguard::guard(f, |f| {
        // ensure we flush file at return or panic
        let _ = f.sync_all();
    });
    // Access the file through the scope guard itself
    file.write_all(b"test me\n").map(|_| ())
}

fn guard_with_defer() {
    // use a cell to observe drops during and after the scope guard is active
    let drop_counter = Cell::new(0);
    {
        // Create a scope guard using `defer!` for the current scope
        defer! {
            dbg!("Adding one");
            drop_counter.set(1 + drop_counter.get());
        }

        // Do regular operations here in the meantime.

        // Just before scope exit: it hasn't run yet.
        dbg!("Original one");
        assert_eq!(drop_counter.get(), 0);

        // The following scope end is where the defer closure is called
    }
    assert_eq!(drop_counter.get(), 1);
}

fn main() {
    guard_file().unwrap();
    guard_with_defer();
}
