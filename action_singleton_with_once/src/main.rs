use std::{
    mem::MaybeUninit,
    sync::{Mutex, Once},
    thread,
};

// 注意这里不能pub
#[derive(Debug)]
struct Config {
    config_str: String,
}

// 单例函数
// 第一 唯一
// 第二 用Maybeunit创建 未被初始化的内存
// 第三 返回&‘static
fn single_config() -> &'static Mutex<Config> {
    static mut CONFIG: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONFIG.as_mut_ptr().write(Mutex::new(Config {
            config_str: "test config".to_string(),
        }));
    });

    unsafe { &*CONFIG.as_ptr() }
}

fn main() {
    let config1 = single_config();
    let config2 = single_config();

    println!("{:p}", config1);
    println!("{:p}", config2);

    // 改变了config1
    {
        let mut conf = config1.lock().unwrap();

        conf.config_str = "config1".to_string();
    }
    // 观察config2是否变化
    println!("{:?}", config1);
    println!("{:?}", config2);

    // 通过子线程 改变了config2
    let handle = thread::spawn(move || {
        let mut conf2 = config2.lock().unwrap();

        conf2.config_str = "config thread".to_string();
    });
    handle.join().unwrap();
    // 观察config1 是否变化
    let config3 = single_config();
    println!("{:?}", config1);
    println!("{:?}", config2);
    println!("{:?}", config3);
}
