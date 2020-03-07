use std::cell::RefCell;

fn main() {
    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;
    shared1.borrow_mut().push(4);
    println!("{:?}", shared_vec.borrow());
    shared2.borrow_mut().push(5);
    println!("{:?}", shared_vec.borrow());
}
