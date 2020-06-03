extern crate access_queue;
extern crate async_std;

use access_queue::AccessQueue;

#[derive(Debug)]
struct Depth {
    x: u16,
    y: u16,
}

#[async_std::main]
async fn main() {
    // This queue limits the number of simultaneous references to `inner` to 4
    let queue = AccessQueue::new(Depth { x: 8, y: 54 }, 4);

    // get an inner reference
    let inner1 = queue.access().await;
    println!("{:?}", inner1);

    // get more (from other concurrent tasks)
    let inner2 = queue.access().await;
    println!("{:?}", inner2);
    let inner3 = queue.access().await;
    println!("{:?}", inner3);
    let inner4 = queue.access().await;
    println!("{:?}", inner4);

    // this access has to wait if no release, because there are already 4 accesses ongoing
    // (note: you should not call access multiple times from within the same
    //  scope. this example is meant to simulate accessing from multiple
    //  tasks concurrently)
    queue.release(1);
    let inner5 = queue.access().await;
    println!("{:?}", inner5);
}
