use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(f: T) -> Cacher<T> {
        Cacher {
            calculation: f,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let s = 3;
    let mut cache = Cacher::new(move |v| {
        println!("doing math ...");
        thread::sleep(Duration::from_secs(s));
        v * 2
    });
    println!("value: {}", cache.value(2));
    println!("value: {}", cache.value(2));
}
