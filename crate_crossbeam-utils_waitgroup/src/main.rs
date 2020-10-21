use crossbeam_utils::sync::WaitGroup;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建新的等待组
    let wg = WaitGroup::new();

    for _ in 0..4 {
        // 创建等待组的引用
        let wg = wg.clone();

        thread::spawn(move || {
            // 实际逻辑
            thread::sleep(Duration::from_secs(3));
            // 解除对等待组的引用
            drop(wg);
        });
    }

    // 会阻塞直到所有线程完成工作
    wg.wait();
}
