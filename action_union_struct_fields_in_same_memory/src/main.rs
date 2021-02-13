union IntOrFloat {
    i: u32,
    f: f32,
}

fn main() {
    let mut u = IntOrFloat { f: 1.0 };
    // Reading the fields of an union is always unsafe
    assert_eq!(unsafe { u.i }, 1065353216);
    // Updating through any of the field will modify all of them
    u.i = 1073741824;
    assert_eq!(unsafe { u.f }, 2.0);

    // Matching
    unsafe {
        match u {
            IntOrFloat { i: 10 } => println!("Found exactly ten!"),
            // Matching the field `f` provides an `f32`.
            IntOrFloat { f } => println!("Found f = {} !", f),
        }
    }

    // Reference
    let f = unsafe { &u.f };
    println!("f = {}", f);
    // This will not compile because the field has already been borrowed, even
    // if only immutably
    // let i = unsafe { &mut u.i };
    // *i = 10;
    // println!("f = {} and i = {}", f, i);
}
