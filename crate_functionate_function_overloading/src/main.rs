#![feature(
    unboxed_closures,
    fn_traits,
    type_alias_impl_trait,
    impl_trait_in_bindings
)]
use functionate::functionate;
use std::ops::Fn;

#[allow(non_upper_case_globals)]
static foo: impl Fn() -> &'static str + Fn(i32) -> i32 = {
    struct Foo;
    #[functionate]
    impl Foo {
        fn a(&self) -> &'static str {
            "bar"
        }
        fn b(&self, x: i32) -> i32 {
            x * 2
        }
    }
    Foo
};

fn main() {
    println!("{}", foo());
    println!("{}", foo(5));
}
