use std::any::{Any, TypeId};

fn is_string(s: &dyn Any) {
    if s.is::<String>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}

fn is_string2<T: 'static>(_: &T) {
    if TypeId::of::<String>() == TypeId::of::<T>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}

fn modify_if_u32(s: &mut dyn Any) {
    if let Some(num) = s.downcast_mut::<u32>() {
        *num = 42;
    }
}

fn main() {
    let s = "Hello, world!".to_string();
    is_string(&0);
    is_string(&s);
    is_string2(&s);
    let mut n = 10u32;
    modify_if_u32(&mut n);
    println!("After modification: {}", n);
}
