#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
macro_rules! hashmap {
    ($ ($ key : expr => $ val : expr), *) =>
    {
        {
            let mut map = :: std :: collections :: HashMap :: new () ; $
            (map . insert ($ key, $ val) ;) * map
        }
    }
}

fn main() {
    let counts = {
        let mut map = ::std::collections::HashMap::new();
        map.insert("A", 0);
        map.insert("C", 1);
        map.insert("E", 2);
        map
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&counts,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
}
