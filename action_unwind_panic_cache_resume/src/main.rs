use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());

    // Catch unwind panic
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());

    // Resume unwind panic
    if let Err(err) = result {
        panic::resume_unwind(err);
    }
}
