extern crate infer;
extern crate mime;

use std::any;

fn print_type<T>(_: T) {
    println!("{}", any::type_name::<T>());
}

fn main() {
    let v = vec![0xFF, 0xD8, 0xFF, 0xAA];
    let info = infer::Infer::new();
    assert_eq!("image/jpeg", info.get(&v).unwrap().mime);
    assert_eq!("jpg", info.get(&v).unwrap().ext);
    assert_eq!(
        "image/png",
        info.get_from_path("data/test").unwrap().unwrap().mime
    );
    print_type(mime::IMAGE_PNG);
    println!("{}", mime::IMAGE_PNG.type_().as_str());
    println!("{}", mime::IMAGE_PNG.subtype().as_str());
    println!("{}", mime::IMAGE_PNG.essence_str());
    assert_eq!(
        mime::IMAGE_PNG,
        info.get_from_path("data/test")
            .unwrap()
            .unwrap()
            .mime
            .as_str()
    );
    assert_eq!("png", info.get_from_path("data/test").unwrap().unwrap().ext);
}
