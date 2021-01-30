// Examples where const generic parameters can be used.
// url: https://doc.rust-lang.org/nightly/reference/items/generics.html#const-generics

// Used in the signature of the item itself.
fn foo<const N: usize>(arr: [i32; N]) {
    // Used as a type within a function body.
    let x: [i32; N];
    // Used as an expression.
    println!("{}", N * 2);
}

// Used as a field of a struct.
struct Foo<const N: usize>([i32; N]);

impl<const N: usize> Foo<N> {
    // Used as an associated constant.
    const CONST: usize = N * 4;
}

trait Trait {
    type Output;
}

impl<const N: usize> Trait for Foo<N> {
    // Used as an associated type.
    type Output = [i32; N];
}

fn double<const N: i32>() {
    println!("doubled: {}", N * 2);
}

const SOME_CONST: i32 = 12;

fn example() {
    // Example usage of a const argument.
    double::<9>();
    double::<-123>();
    double::<{ 7 + 8 }>();
    double::<SOME_CONST>();
    double::<{ SOME_CONST + 5 }>();
}

fn main() {}

/* Examples where const parameters may not be used. */

// Not allowed to combine in other expressions in types, such as the
// arithmetic expression in the return type here.
/*
fn bad_function<const N: usize>() -> [u8; { N + 1 }] {
    // Similarly not allowed for array repeat expressions.
    [1; { N + 1 }]
}
*/

// The following is an error, because `N` is interpreted as the type alias `N`.
/*
type N = u32;
struct Bar<const N: usize>;
fn bar<const N: usize>() -> Bar<N> {
    todo!()
}
*/
