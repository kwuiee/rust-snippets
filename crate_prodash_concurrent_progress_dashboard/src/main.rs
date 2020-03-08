extern crate futures;
extern crate prodash;

use futures::task::{LocalSpawnExt, SpawnExt};
use prodash::tui::ticker;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 获取一个进程树
    let root = prodash::Tree::new();
    // 配置gui，为其提供一个不断变化的树的句柄
    let render_fut = prodash::tui::render(
        root.clone(),
        prodash::tui::TuiOptions {
            title: "minimal example".into(),
            ..prodash::tui::TuiOptions::default()
        },
    )?;
    // 由于它会一直不停运行，我们需要一种可以停止它的方法。
    let (render_fut, abort_handle) = futures::future::abortable(render_fut);
    let pool = futures::executor::LocalPool::new();
    // 将gui放入后台
    let gui = pool.spawner().spawn_with_handle(async {
        render_fut.await.ok();
        ()
    })?;
    // …并运行可提供进度的任务
    pool.spawner().spawn_local({
        use futures::StreamExt;
        let mut progress = root.add_child("task");
        async move {
            progress.init(None, Some("items"));
            let mut count = 0;
            let mut ticks = ticker(std::time::Duration::from_millis(100));
            while let Some(_) = ticks.next().await {
                progress.set(count);
                count += 1;
            }
        }
    })?;
    // …完成后，停止gui
    abort_handle.abort();
    //…然后等待运行结束
    futures::executor::block_on(gui);
    Ok(())
}
