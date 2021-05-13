echo "test" | cargo run
cargo run | grep std
cargo run 2>&1 | grep std
