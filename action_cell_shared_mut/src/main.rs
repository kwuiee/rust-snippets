use std::cell::Cell;

fn main() {
    let data: Cell<i32> = Cell::new(0);
    let p = &data;
    data.set(10);
    println!("{}", p.get());
    p.set(20);
    println!("{:?}", data);
}
