use async_std::task::sleep;
use futures::join;

use std::fmt::Write;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a default dynamic configuration
    let pager = minus::Pager::new().finish();

    // Asynchronously push numbers to the output
    let increment = async {
        for i in 0..=100_u32 {
            let mut guard = pager.lock().unwrap();
            writeln!(guard.lines, "{}", i)?;
            drop(guard);
            sleep(Duration::from_millis(100)).await;
        }
        // Return an Ok result
        Result::<_, std::fmt::Error>::Ok(())
    };
    // Join the futures
    let (res1, res2) = join!(minus::async_std_updating(pager.clone()), increment);

    // Check for errors
    res1?;
    res2?;
    // Return Ok result
    Ok(())
}
