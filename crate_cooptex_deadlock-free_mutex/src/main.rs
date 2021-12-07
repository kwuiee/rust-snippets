use cooptex::*;

fn main() {
    let a = CoopMutex::new(42);
    let b = CoopMutex::new(43);

    retry_loop(|| {
        let a_lock = a.lock()?.unwrap();
        let b_lock = b.lock()?.unwrap();
        assert_eq!(*a_lock + *b_lock, 85);
        Ok(())
    });
}
