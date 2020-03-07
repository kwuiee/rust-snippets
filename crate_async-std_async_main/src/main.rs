use async_std::task;
// ^ we need this for task spawning

async fn negate_async(n: i32) -> i32 {
    println!("Negating {}", n);
    task::sleep(std::time::Duration::from_secs(5)).await;
    println!("Finished sleeping for {}!", n);
    n * -1
}

async fn f() -> i32 {
    let neg = negate_async(1);
    // ... nothing happens yet
    let neg_task = task::spawn(negate_async(2));
    // ^ this task /is/ started
    task::sleep(std::time::Duration::from_secs(1)).await;
    // we sleep for effect.

    neg.await + neg_task.await
    // ^ this starts the first task `neg`
    // and waits for both tasks to finish
}

fn main() {
    let child = task::block_on(f());
    println!("{}", child);
}
