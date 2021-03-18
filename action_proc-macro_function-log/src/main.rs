use func_log_derive::func_log;

fn main() {
    shortest();
    let a = "short";
    let b = "longer";
    let ret = unsafe { longest(a, b) };
    println!("The longest word is {}.", ret);
}

// 叠几个BUFF试试
#[func_log]
pub unsafe fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    println!(">> Longest function");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[func_log]
fn shortest() {
    println!(">> Shortest function");
}
