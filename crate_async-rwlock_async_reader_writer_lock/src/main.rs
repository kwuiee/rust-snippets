use async_rwlock::RwLock;

#[async_std::main]
async fn main() {
    let lock = RwLock::new(5);

    // Multiple read locks can be held at a time.
    let r1 = lock.read().await;
    let r2 = lock.read().await;
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
    drop((r1, r2));

    // Only one write lock can be held at a time.
    let mut w = lock.write().await;
    *w += 1;
    assert_eq!(*w, 6);
}