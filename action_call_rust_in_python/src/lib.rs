#[no_mangle]
pub extern "C" fn fibonacci(n: u32) -> u64 {
    if n > 1 {
        fibonacci(n - 1) + fibonacci(n - 2)
    } else {
        1
    }
}
