use std::any;

fn print_type<T>(_: T) {
    println!("{}", any::type_name::<T>());
}

fn main() {
    print_type("dsaf");
}
