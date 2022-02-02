use include_dir::{include_dir, Dir};
use std::path::Path;

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

fn main() {
    // of course, you can retrieve a file by its full path
    let lib_rs = PROJECT_DIR.get_file("src/main.rs").unwrap();

    // you can also inspect the file's contents
    let body = lib_rs.contents_utf8().unwrap();
    assert!(body.contains("SOME_INTERESTING_STRING"));

    // if you enable the `glob` feature, you can for files (and directories) using glob patterns
    #[cfg(feature = "glob")]
    {
        let glob = "**/*.rs";
        for entry in PROJECT_DIR.find(glob).unwrap() {
            println!("Found {}", entry.path().display());
        }
    }
}
