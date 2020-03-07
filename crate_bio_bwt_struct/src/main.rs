extern crate bio;

use bio::data_structures::bwt::bwt;
use bio::data_structures::suffix_array::suffix_array;

fn main() {
    let text = b"banana$";
    let pos = suffix_array(text);
    let bwt = bwt(text, &pos);
    println!("{}", String::from_utf8(bwt).unwrap());
}
