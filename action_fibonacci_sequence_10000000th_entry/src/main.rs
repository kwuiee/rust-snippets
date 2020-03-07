use std::env;

fn fibo1(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibo1(n - 1) + fibo1(n - 2)
    }
}

fn fibo2(n: u64) -> u64 {
    let mut s = 1u64;
    let mut t = 1u64;
    for _ in 0..n {
        let mid = s;
        s = t + s;
        t = mid;
    }
    s
}

fn main() {
    let nth = env::args().nth(1).unwrap().parse::<u64>().unwrap();
    println!("{} fibo number: {}", nth, fibo2(nth));
}

#[cfg(test)]
mod test_all {
    use super::*;
    #[test]
    fn test1() {
        println!("{} fibo number: {}", 50, fibo1(50));
    }
    #[test]
    fn test2() {
        println!("{} fibo number: {}", 50, fibo2(50));
    }
}
