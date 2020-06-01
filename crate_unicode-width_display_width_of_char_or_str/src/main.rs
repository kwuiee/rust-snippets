extern crate unicode_width;

use unicode_width::UnicodeWidthStr;

fn main() {
    let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
    let width = UnicodeWidthStr::width(teststr);
    println!("{}", teststr);
    println!("The above string is {} columns wide.", width);
    let width = teststr.width_cjk();
    println!("The above string is {} columns wide (CJK).", width);
}